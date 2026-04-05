# Step 3: 納品手順書（Google Cloud Run デプロイ）

---

## 1. 概要

### 1.1 目的
本手順書は、資格管理・目標設定システム（CertManager）をGoogle Cloud Runへデプロイするための手順を記載する。

### 1.2 デプロイ構成

```
┌─────────────────────────────────────────────────────┐
│                 Google Cloud Run                     │
│                                                      │
│  ┌───────────────────────────────────────────────┐  │
│  │        CertManager コンテナ                     │  │
│  │                                                │  │
│  │  ┌─────────────────────────────────────────┐  │  │
│  │  │  Actix-web (Rust)                        │  │  │
│  │  │  - API エンドポイント (/api/*)             │  │  │
│  │  │  - 静的ファイル配信 (Vue ビルド成果物)      │  │  │
│  │  └─────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────┘  │
│                       │                              │
│              HTTPS (マネージド証明書)                  │
└───────────────────────┼─────────────────────────────┘
                        │
              ┌─────────▼──────────┐
              │     Supabase        │
              │   (PostgreSQL)      │
              └────────────────────┘
```

### 1.3 前提条件

| 項目 | 要件 |
|------|------|
| Google Cloud アカウント | プロジェクト作成済み、課金有効 |
| gcloud CLI | v450.0.0 以上がインストール済み |
| Docker | v24.0 以上がインストール済み |
| Node.js | v20 LTS 以上がインストール済み |
| Rust | 1.78 以上がインストール済み |
| Supabase | プロジェクト作成済み、REST API URL と anon key を取得済み |

---

## 2. 事前準備

### 2.1 Google Cloud プロジェクトの設定

```bash
# Google Cloud にログイン
gcloud auth login

# プロジェクトを設定（プロジェクトIDを置換）
export PROJECT_ID="your-project-id"
gcloud config set project $PROJECT_ID

# 必要なAPIの有効化
gcloud services enable \
  run.googleapis.com \
  containerregistry.googleapis.com \
  artifactregistry.googleapis.com \
  cloudbuild.googleapis.com

# Artifact Registry にDockerリポジトリを作成
gcloud artifacts repositories create certmanager-repo \
  --repository-format=docker \
  --location=asia-northeast1 \
  --description="CertManager Docker images"
```

### 2.2 Supabase データベースの初期化

Supabase管理画面のSQL Editorで以下を順番に実行する。

#### 2.2.1 拡張機能の有効化
```sql
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
```

#### 2.2.2 テーブル作成
```sql
-- TBL_USER
CREATE TABLE "TBL_USER" (
    useid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    usenm VARCHAR(30) NOT NULL,
    useml VARCHAR(128) NOT NULL UNIQUE,
    usepw VARCHAR(256) NOT NULL,
    useca TIMESTAMPTZ NOT NULL DEFAULT now(),
    useua TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TBL_MASTER
CREATE TABLE "TBL_MASTER" (
    masid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    masnm VARCHAR(200) NOT NULL,
    masct VARCHAR(30) NOT NULL DEFAULT 'その他',
    masnr VARCHAR(200) NOT NULL UNIQUE,
    masca TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TBL_HOLDING
CREATE TABLE "TBL_HOLDING" (
    holid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    holui UUID NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,
    holmi UUID NOT NULL REFERENCES "TBL_MASTER"(masid),
    holdt DATE,
    holca TIMESTAMPTZ NOT NULL DEFAULT now(),
    holua TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TBL_GOAL
CREATE TABLE "TBL_GOAL" (
    goaid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    goaui UUID NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,
    goami UUID NOT NULL REFERENCES "TBL_MASTER"(masid),
    goatd DATE NOT NULL,
    goast VARCHAR(20) NOT NULL DEFAULT 'studying'
        CHECK (goast IN ('studying', 'scheduled', 'achieved', 'suspended')),
    goamm TEXT,
    goaca TIMESTAMPTZ NOT NULL DEFAULT now(),
    goaua TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TBL_FAVORITE
CREATE TABLE "TBL_FAVORITE" (
    favid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    favui UUID NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,
    favti UUID NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,
    favca TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (favui, favti),
    CHECK (favui != favti)
);

-- TBL_SESSION
CREATE TABLE "TBL_SESSION" (
    sesid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sesui UUID NOT NULL REFERENCES "TBL_USER"(useid) ON DELETE CASCADE,
    sestk VARCHAR(128) NOT NULL UNIQUE,
    sesea TIMESTAMPTZ NOT NULL,
    sesca TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

#### 2.2.3 インデックス作成
```sql
CREATE INDEX idx_holding_user ON "TBL_HOLDING"(holui);
CREATE INDEX idx_goal_user ON "TBL_GOAL"(goaui);
CREATE INDEX idx_goal_status ON "TBL_GOAL"(goast);
CREATE INDEX idx_fav_user ON "TBL_FAVORITE"(favui);
CREATE INDEX idx_session_expiry ON "TBL_SESSION"(sesea);
CREATE INDEX idx_master_name_trgm ON "TBL_MASTER" USING GIN (masnm gin_trgm_ops);
```

#### 2.2.4 更新トリガー作成
```sql
-- TBL_USER
CREATE OR REPLACE FUNCTION update_user_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.useua = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_user_updated
    BEFORE UPDATE ON "TBL_USER"
    FOR EACH ROW EXECUTE FUNCTION update_user_updated_at();

-- TBL_HOLDING
CREATE OR REPLACE FUNCTION update_holding_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.holua = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_holding_updated
    BEFORE UPDATE ON "TBL_HOLDING"
    FOR EACH ROW EXECUTE FUNCTION update_holding_updated_at();

-- TBL_GOAL
CREATE OR REPLACE FUNCTION update_goal_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.goaua = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_goal_updated
    BEFORE UPDATE ON "TBL_GOAL"
    FOR EACH ROW EXECUTE FUNCTION update_goal_updated_at();
```

#### 2.2.5 RPC関数作成
```sql
CREATE OR REPLACE FUNCTION get_community_users(
    p_user_id UUID,
    p_limit INT DEFAULT 20,
    p_offset INT DEFAULT 0
)
RETURNS TABLE (
    user_id UUID,
    username VARCHAR,
    certification_count BIGINT,
    goal_count BIGINT,
    achieved_count BIGINT,
    has_good_mark BOOLEAN,
    is_favorite BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        u.useid AS user_id,
        u.usenm AS username,
        COALESCE(h.cnt, 0) AS certification_count,
        COALESCE(g.cnt, 0) AS goal_count,
        COALESCE(a.cnt, 0) AS achieved_count,
        COALESCE(a.cnt, 0) > 0 AS has_good_mark,
        f.favid IS NOT NULL AS is_favorite
    FROM "TBL_USER" u
    LEFT JOIN (
        SELECT holui, COUNT(*) AS cnt FROM "TBL_HOLDING" GROUP BY holui
    ) h ON u.useid = h.holui
    LEFT JOIN (
        SELECT goaui, COUNT(*) AS cnt FROM "TBL_GOAL" GROUP BY goaui
    ) g ON u.useid = g.goaui
    LEFT JOIN (
        SELECT goaui, COUNT(*) AS cnt FROM "TBL_GOAL"
        WHERE goast = 'achieved' GROUP BY goaui
    ) a ON u.useid = a.goaui
    LEFT JOIN "TBL_FAVORITE" f ON f.favui = p_user_id AND f.favti = u.useid
    WHERE u.useid != p_user_id
    ORDER BY is_favorite DESC, certification_count DESC
    LIMIT p_limit OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;
```

#### 2.2.6 初期マスタデータの投入
```sql
INSERT INTO "TBL_MASTER" (masnm, masct, masnr) VALUES
('基本情報技術者', 'IT', '基本情報技術者'),
('応用情報技術者', 'IT', '応用情報技術者'),
('情報セキュリティマネジメント', 'IT', '情報セキュリティマネジメント'),
('データベーススペシャリスト', 'IT', 'データベーススペシャリスト'),
('ネットワークスペシャリスト', 'IT', 'ネットワークスペシャリスト'),
('AWS Solutions Architect Associate', 'IT', 'awssolutionsarchitectassociate'),
('AWS Solutions Architect Professional', 'IT', 'awssolutionsarchitectprofessional'),
('TOEIC', '語学', 'toeic'),
('TOEFL iBT', '語学', 'toeflibt'),
('英検1級', '語学', '英検1級'),
('英検準1級', '語学', '英検準1級'),
('日商簿記2級', '金融', '日商簿記2級'),
('日商簿記3級', '金融', '日商簿記3級'),
('FP2級', '金融', 'fp2級'),
('FP3級', '金融', 'fp3級'),
('宅地建物取引士', '法律', '宅地建物取引士'),
('行政書士', '法律', '行政書士');
```

#### 2.2.7 Supabase RLS（Row Level Security）の設定

Supabaseではデフォルトで新規テーブルにRLSが有効になるため、REST API経由のアクセスを許可する設定が必要。本システムではバックエンド（Rust）がservice_roleキーでSupabaseにアクセスする構成とする。

```sql
-- RLSをバイパスするため、service_role keyを使用する場合は
-- RLSポリシーの設定は不要（service_roleは全テーブルにフルアクセス可能）

-- ただし安全のため、anon keyでのアクセスを遮断する
ALTER TABLE "TBL_USER" ENABLE ROW LEVEL SECURITY;
ALTER TABLE "TBL_MASTER" ENABLE ROW LEVEL SECURITY;
ALTER TABLE "TBL_HOLDING" ENABLE ROW LEVEL SECURITY;
ALTER TABLE "TBL_GOAL" ENABLE ROW LEVEL SECURITY;
ALTER TABLE "TBL_FAVORITE" ENABLE ROW LEVEL SECURITY;
ALTER TABLE "TBL_SESSION" ENABLE ROW LEVEL SECURITY;

-- anon roleには何もアクセスさせない（ポリシーなし = アクセス拒否）
-- service_role keyを使うRustバックエンドは RLS をバイパスする
```

> **重要**: `SUPABASE_KEY` 環境変数にはanon keyではなく `service_role` キーを設定すること。Supabase管理画面の Settings > API から取得する。

### 2.3 環境変数の準備

以下の値を事前に確認・準備しておく。

| 変数名 | 取得方法 |
|--------|---------|
| SUPABASE_URL | Supabase管理画面 > Settings > API > Project URL + `/rest/v1` |
| SUPABASE_KEY | Supabase管理画面 > Settings > API > service_role key |
| EMAIL_HMAC_SECRET | `openssl rand -hex 32` で生成（32バイトの乱数） |
| CORS_ORIGIN | Cloud RunのサービスURL（初回デプロイ後に確認） |

```bash
# EMAIL_HMAC_SECRET の生成
openssl rand -hex 32
# 出力例: a1b2c3d4e5f6789...（64文字のHex文字列）
```

---

## 3. ビルド手順

### 3.1 リポジトリのクローン

```bash
git clone https://github.com/your-org/certmanager.git
cd certmanager
```

### 3.2 フロントエンドのビルド

```bash
cd frontend

# 依存パッケージのインストール
npm ci

# 環境変数の設定（API のベースURLはプロキシ経由のため空文字でよい）
cat > .env.production << 'EOF'
VITE_API_BASE_URL=
EOF

# プロダクションビルド
npm run build

# ビルド成果物の確認
ls -la dist/
# index.html, assets/ ディレクトリが存在すること

cd ..
```

### 3.3 バックエンドのビルド（ローカル確認用）

```bash
cd backend

# 依存crateのダウンロードとビルド（ローカル確認用）
cargo build --release

# ビルド成果物の確認
ls -la target/release/certmanager

cd ..
```

### 3.4 Dockerイメージのビルド

プロジェクトルートにある統合Dockerfileを使用する。

```bash
# Dockerfile の確認
cat Dockerfile
```

**Dockerfile（プロジェクトルート）**:
```dockerfile
# ===== Stage 1: フロントエンドビルド =====
FROM node:20-slim AS frontend-builder
WORKDIR /app/frontend

COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# ===== Stage 2: バックエンドビルド =====
FROM rust:1.78-slim AS backend-builder
WORKDIR /app

# 依存キャッシュ用にCargo.tomlとCargo.lockだけ先にコピー
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm -rf src

# ソースコードをコピーしてフルビルド
COPY backend/src/ src/
RUN touch src/main.rs && cargo build --release

# ===== Stage 3: 実行環境 =====
FROM debian:bookworm-slim AS runtime

# CA証明書のインストール（HTTPS通信に必要）
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 非rootユーザーの作成
RUN useradd -m -s /bin/bash appuser

WORKDIR /app

# バックエンドバイナリのコピー
COPY --from=backend-builder /app/target/release/certmanager /app/certmanager

# フロントエンドビルド成果物のコピー
COPY --from=frontend-builder /app/frontend/dist /app/static

# 所有者変更
RUN chown -R appuser:appuser /app

USER appuser

# Cloud Runで使用するポート
EXPOSE 8080

# 環境変数のデフォルト
ENV SERVER_PORT=8080
ENV RUST_LOG=info
ENV STATIC_DIR=/app/static

CMD ["/app/certmanager"]
```

```bash
# Dockerイメージのビルド
docker build -t certmanager:latest .

# ローカルでの動作確認（オプション）
docker run -p 8080:8080 \
  -e SUPABASE_URL="https://xxx.supabase.co/rest/v1" \
  -e SUPABASE_KEY="your-service-role-key" \
  -e EMAIL_HMAC_SECRET="your-secret" \
  -e CORS_ORIGIN="http://localhost:8080" \
  certmanager:latest

# ブラウザで http://localhost:8080 にアクセスして動作確認
```

---

## 4. デプロイ手順

### 4.1 Artifact Registry へのイメージプッシュ

```bash
# Docker認証の設定
gcloud auth configure-docker asia-northeast1-docker.pkg.dev

# イメージにタグ付け
export IMAGE_TAG="asia-northeast1-docker.pkg.dev/$PROJECT_ID/certmanager-repo/certmanager:v1.0.0"
docker tag certmanager:latest $IMAGE_TAG

# プッシュ
docker push $IMAGE_TAG

# プッシュされたことを確認
gcloud artifacts docker images list asia-northeast1-docker.pkg.dev/$PROJECT_ID/certmanager-repo
```

### 4.2 Cloud Run へのデプロイ

```bash
# 初回デプロイ
gcloud run deploy certmanager \
  --image $IMAGE_TAG \
  --region asia-northeast1 \
  --platform managed \
  --allow-unauthenticated \
  --port 8080 \
  --memory 512Mi \
  --cpu 1 \
  --min-instances 1 \
  --max-instances 10 \
  --timeout 300 \
  --set-env-vars "\
SUPABASE_URL=https://xxx.supabase.co/rest/v1,\
SUPABASE_KEY=your-service-role-key,\
EMAIL_HMAC_SECRET=your-hmac-secret,\
CORS_ORIGIN=https://certmanager-xxxx-an.a.run.app,\
RUST_LOG=info"
```

> **注意**: 初回デプロイ時は CORS_ORIGIN にダミー値を設定し、デプロイ後にサービスURLを確認して更新する。

### 4.3 CORS_ORIGINの更新（初回デプロイ後）

```bash
# サービスURLの確認
gcloud run services describe certmanager \
  --region asia-northeast1 \
  --format "value(status.url)"
# 出力例: https://certmanager-abc123-an.a.run.app

# CORS_ORIGINを実際のURLに更新
gcloud run services update certmanager \
  --region asia-northeast1 \
  --set-env-vars "CORS_ORIGIN=https://certmanager-abc123-an.a.run.app"
```

### 4.4 デプロイ後の動作確認

```bash
# サービスURLを変数に設定
export SERVICE_URL=$(gcloud run services describe certmanager \
  --region asia-northeast1 \
  --format "value(status.url)")

# 1. ヘルスチェック（フロントエンドが配信されるか）
curl -s -o /dev/null -w "%{http_code}" $SERVICE_URL
# 期待結果: 200

# 2. APIエンドポイントの確認
curl -s -o /dev/null -w "%{http_code}" $SERVICE_URL/api/auth/me
# 期待結果: 401（未認証のため）

# 3. サインアップのテスト
curl -X POST $SERVICE_URL/api/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"テスト","email":"deploy-test@example.com","password":"TestPass1!"}'
# 期待結果: 201

# 4. サインインのテスト
curl -X POST $SERVICE_URL/api/auth/signin \
  -H "Content-Type: application/json" \
  -c cookies.txt \
  -d '{"email":"deploy-test@example.com","password":"TestPass1!"}'
# 期待結果: 200 + cookies.txt にセッショントークン

# 5. 認証付きAPIの確認
curl -b cookies.txt $SERVICE_URL/api/auth/me
# 期待結果: 200 + ユーザー情報JSON

# 6. テストデータのクリーンアップ（Supabase SQL Editorで実行）
# DELETE FROM "TBL_USER" WHERE usenm = 'テスト';
```

### 4.5 ブラウザでの最終確認

1. ブラウザでサービスURLにアクセス
2. サインアップ画面が表示されることを確認
3. 新規ユーザーでサインアップ→サインイン→各画面の動作確認
4. HTTPS接続であることを確認（ブラウザの鍵マーク）
5. Cookieの属性が正しいことをDevToolsで確認（HttpOnly, Secure, SameSite=Strict）

---

## 5. 運用設定

### 5.1 ログの確認

```bash
# Cloud Run のログを確認
gcloud logging read "resource.type=cloud_run_revision AND resource.labels.service_name=certmanager" \
  --limit 50 \
  --format "table(timestamp, jsonPayload.message, jsonPayload.level)"

# リアルタイムログのストリーミング
gcloud alpha run services logs tail certmanager --region asia-northeast1
```

### 5.2 Cloud Run のモニタリング

Google Cloud Console > Cloud Run > certmanager から以下を監視する。

| メトリクス | 確認事項 |
|----------|---------|
| リクエスト数 | 異常なスパイクがないか |
| レイテンシ | 95パーセンタイルが500ms以内か |
| エラー率 | 5xxエラーが発生していないか |
| インスタンス数 | オートスケーリングが適切に動作しているか |
| メモリ使用率 | 512MBの上限に近づいていないか |

### 5.3 アラートの設定（推奨）

```bash
# エラー率のアラートポリシー作成（Cloud Monitoring）
# 5分間のエラー率が5%を超えた場合に通知
gcloud alpha monitoring policies create \
  --display-name="CertManager Error Rate Alert" \
  --condition-display-name="High Error Rate" \
  --condition-filter='resource.type="cloud_run_revision" AND metric.type="run.googleapis.com/request_count" AND metric.labels.response_code_class="5xx"' \
  --condition-threshold-value=5 \
  --condition-threshold-duration=300s \
  --notification-channels="projects/$PROJECT_ID/notificationChannels/YOUR_CHANNEL_ID"
```

### 5.4 セッションクリーンアップ（定期実行）

期限切れセッションの削除をSupabaseのcronジョブで定期実行する。

Supabase管理画面のSQL Editorで以下を実行:

```sql
-- pg_cron拡張の有効化（Supabaseで利用可能）
CREATE EXTENSION IF NOT EXISTS pg_cron;

-- 毎日午前3時（UTC）に期限切れセッションを削除
SELECT cron.schedule(
    'cleanup-expired-sessions',
    '0 3 * * *',
    $$DELETE FROM "TBL_SESSION" WHERE sesea < now()$$
);
```

---

## 6. バージョンアップ手順

### 6.1 新バージョンのデプロイ

```bash
# 1. ソースコードの更新
git pull origin main

# 2. フロントエンドの再ビルド（変更がある場合）
cd frontend && npm ci && npm run build && cd ..

# 3. Dockerイメージの再ビルド
export NEW_VERSION="v1.1.0"
docker build -t certmanager:$NEW_VERSION .

# 4. イメージのタグ付けとプッシュ
export NEW_IMAGE_TAG="asia-northeast1-docker.pkg.dev/$PROJECT_ID/certmanager-repo/certmanager:$NEW_VERSION"
docker tag certmanager:$NEW_VERSION $NEW_IMAGE_TAG
docker push $NEW_IMAGE_TAG

# 5. Cloud Run の更新
gcloud run deploy certmanager \
  --image $NEW_IMAGE_TAG \
  --region asia-northeast1
```

### 6.2 ロールバック

```bash
# 過去のリビジョン一覧を確認
gcloud run revisions list --service certmanager --region asia-northeast1

# 特定のリビジョンにトラフィックを100%移行
gcloud run services update-traffic certmanager \
  --region asia-northeast1 \
  --to-revisions REVISION_NAME=100
```

### 6.3 DBマイグレーション

テーブル構造の変更が必要な場合、以下の手順で実施する。

1. Supabase SQL Editorでマイグレーションスクリプトを実行
2. バックエンドの新バージョンをデプロイ
3. 動作確認
4. 問題がある場合はSQLでロールバック + バックエンドをロールバック

> **注意**: カラム削除などの破壊的変更は、まず新コードで旧カラムと新カラムの両方をサポートする中間リリースを行い、完全移行後に旧カラムを削除する二段階デプロイを推奨する。

---

## 7. トラブルシューティング

### 7.1 よくある問題と対処法

| 問題 | 原因 | 対処法 |
|------|------|--------|
| コンテナが起動しない | 環境変数の不足 | `gcloud run services describe` で環境変数を確認 |
| 502 Bad Gateway | アプリケーションが8080ポートでリッスンしていない | Dockerfile の EXPOSE と CMD を確認 |
| CORS エラー | CORS_ORIGIN が正しくない | Cloud Run のサービスURLと CORS_ORIGIN を照合 |
| DB接続エラー | SUPABASE_URL / SUPABASE_KEY が不正 | Supabase管理画面から正しい値を再取得 |
| サインインできない | EMAIL_HMAC_SECRET が変更された | シークレットキーの変更は全ユーザーのメールハッシュを無効化するため、絶対に変更しない |
| メモリ超過 | Argon2idのメモリコストが高すぎる | Cloud Run のメモリ割り当てを増やす（1Gi以上推奨） |
| コールドスタートが遅い | Rustバイナリの初期化 | min-instances=1 に設定済みか確認 |
| ビルドが遅い | Rustコンパイル | Docker buildx のキャッシュ利用を検討 |

### 7.2 ログによるデバッグ

```bash
# エラーログのみ抽出
gcloud logging read \
  "resource.type=cloud_run_revision AND resource.labels.service_name=certmanager AND jsonPayload.level=ERROR" \
  --limit 20 \
  --format json

# 特定の時間範囲のログ
gcloud logging read \
  "resource.type=cloud_run_revision AND resource.labels.service_name=certmanager AND timestamp>=\"2025-01-01T00:00:00Z\"" \
  --limit 50
```

### 7.3 緊急時の対応

```bash
# サービスの一時停止（全トラフィックを遮断）
gcloud run services update certmanager \
  --region asia-northeast1 \
  --no-traffic

# サービスの再開
gcloud run services update-traffic certmanager \
  --region asia-northeast1 \
  --to-latest
```

---

## 8. チェックリスト

### 8.1 デプロイ前チェックリスト

- [ ] Supabase のテーブル・インデックス・トリガー・RPC関数が作成されていること
- [ ] Supabase の RLS が有効化されていること
- [ ] 初期マスタデータが投入されていること
- [ ] `EMAIL_HMAC_SECRET` が生成され、安全に保管されていること
- [ ] Supabase の `service_role` キーを取得していること
- [ ] Google Cloud プロジェクトの API が有効化されていること
- [ ] Artifact Registry リポジトリが作成されていること
- [ ] Docker イメージのビルドが成功していること
- [ ] ローカルでの動作確認が完了していること

### 8.2 デプロイ後チェックリスト

- [ ] サービスURL にブラウザでアクセスし、フロントエンドが表示されること
- [ ] サインアップが正常に完了すること
- [ ] サインインが正常に完了し、Cookie が設定されること
- [ ] Cookie に HttpOnly, Secure, SameSite=Strict が設定されていること
- [ ] 所持資格の登録・一覧・編集・削除が正常に動作すること
- [ ] 目標の登録・ステータス変更が正常に動作すること
- [ ] 資格名の入力補完が動作すること
- [ ] コミュニティ画面にユーザー一覧が表示されること
- [ ] お気に入り機能が正常に動作すること
- [ ] CORS_ORIGIN が正しいサービスURLに設定されていること
- [ ] Cloud Logging にログが出力されていること
- [ ] エラーが発生していないこと
- [ ] レスポンスが HTTPS で配信されていること

### 8.3 セキュリティチェックリスト

- [ ] DB内のパスワードが Argon2id ハッシュであること（直接DBを確認）
- [ ] DB内のメールアドレスが HMAC-SHA256 ハッシュであること（直接DBを確認）
- [ ] `EMAIL_HMAC_SECRET` がコードにハードコーディングされていないこと
- [ ] `SUPABASE_KEY` がコードにハードコーディングされていないこと
- [ ] 他ユーザーのデータにアクセスできないこと（セッション差し替えテスト）
- [ ] 不正なセッショントークンで 401 が返ること
- [ ] ブラウザのDevTools でフロントエンドのソースマップが公開されていないこと
