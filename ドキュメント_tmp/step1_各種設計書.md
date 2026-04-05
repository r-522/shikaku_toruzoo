# Step 1: 要件定義書・基本設計書・詳細設計書

 ---

# 第1部 要件定義書

## 1 . プロジェクト概要

### 1.1 プロジェクト名

資格管理・目標設定システム（CertManager）

### 1.2 目的

ユーザーが保有する資格情報を一元管理し、今後取得を目指す資格への目標設定を行うことで、自己成長を支援するWebアプリケーションを構築する。他ユーザーの状況可視化やお気に入り機能によるモチベーション維持も実現する。

### 1.3 スコープ

* 独自認証によるユーザー管理（サインアップ / サインイン / サインアウト）
* 所持資格の登録・一覧・編集・削除（CRUD）
* 取得目標の設定・管理
* 資格名マスタの自動蓄積と入力補完
* グローバルユーザー一覧の可視化とお気に入り機能
* Google Cloud Run へのデプロイ

### 1.4 対象外スコープ

* Supabase Auth の利用（独自認証を使用する）
* 認可・ロールベースアクセス制御
* フォロー / フォロワーなどのSNS的関係管理
* メール通知・プッシュ通知
* 管理者画面（将来の拡張対象）

 ---

## 2 . ステークホルダー

|区分|説明|
|-|-|
|エンドユーザー|資格取得を目指す社会人・学生|
|開発チーム|本システムの設計・実装を行うエンジニア|
|運用チーム|Cloud Run 上のアプリケーション運用担当|

 ---

## 3 . 機能要件

### FR-001: ユーザー認証

|項目|内容|
|-|-|
|ID|FR-001|
|機能名|ユーザー認証|
|概要|メールアドレスとパスワードによるサインアップ・サインイン・サインアウト|
|詳細|・サインアップ時にユーザー名・メールアドレス・パスワードを入力させる<br>・メールアドレスとパスワードは必ずハッシュ化してDBに保存する<br>・サインイン時はハッシュ化されたメールアドレスで照合を行う<br>・セッションはサインアウトしない限り永続的に保持する<br>・サインアウト時にセッションを無効化する|
|優先度|必須|

### FR-002: 所持資格管理

|項目|内容|
|-|-|
|ID|FR-002|
|機能名|所持資格管理|
|概要|自身が所持する資格の登録・一覧表示・編集・削除|
|詳細|・資格名は入力補完付きテキストフィールドから入力する<br>・取得日を登録できる<br>・登録済み資格の一覧をカード形式で表示する<br>・各資格の編集・削除が可能|
|優先度|必須|

### FR-003: 目標設定

|項目|内容|
|-|-|
|ID|FR-003|
|機能名|目標設定|
|概要|今後取得を目指す資格への目標設定・管理|
|詳細|・資格マスタから資格を選択し、目標日を設定する<br>・目標に対するメモを記録できる<br>・目標ステータス（学習中・受験予定・達成・中断）を管理できる<br>・目標達成時に所持資格へ自動変換するか確認する|
|優先度|必須|

### FR-004: 資格マスタ管理

|項目|内容|
|-|-|
|ID|FR-004|
|機能名|資格マスタ管理|
|概要|資格名のマスタデータベースの自動蓄積と入力補完|
|詳細|・新規資格名が入力された場合、マスタDBに自動追加する<br>・次回以降の入力時にマスタからの候補をサジェストする<br>・カテゴリ（IT・語学・金融・医療・法律・その他）で分類する<br>・重複登録を防止する（正規化した名前で一致判定）|
|優先度|必須|

### FR-005: 他ユーザー状況表示

|項目|内容|
|-|-|
|ID|FR-005|
|機能名|他ユーザー状況表示|
|概要|グローバルな登録ユーザーの資格取得状況をリスト形式で可視化する|
|詳細|・全ユーザーの資格保有数・目標数・最近の達成状況を一覧表示する<br>・一覧は「崩しを入れた」自然なレイアウト（カード幅のバリエーション・余白の非対称性等）で表現する<br>・目標達成実績のあるユーザーに「グッドマーク」バッジを表示する<br>・お気に入り登録されたユーザーは一覧上部に優先表示する|
|優先度|必須|

### FR-006: お気に入り機能

|項目|内容|
|-|-|
|ID|FR-006|
|機能名|お気に入り機能|
|概要|他ユーザーをお気に入り登録し、一覧上部に優先表示する|
|詳細|・ユーザー一覧の各ユーザーに☆マークを配置する<br>・☆マークをクリックするとお気に入りのトグルが可能<br>・お気に入りユーザーは一覧の最上部にソートされる<br>・お気に入り解除も同様のUIで可能|
|優先度|必須|

 ---

## 4 . 非機能要件

### NFR-001: セキュリティ

|項目|内容|
|-|-|
|パスワードハッシュ|Argon2id（メモリ硬化型、GPU/ASIC耐性）|
|メールアドレスハッシュ|HMAC-SHA256（検索可能性を確保するため固定ソルトとしてサーバー管理のシークレットキーを使用）|
|セッション管理|暗号学的に安全な乱数によるトークン生成（256bit）|
|通信|HTTPS（Cloud Runのマネージド証明書）|
|CORS|フロントエンドのオリジンのみ許可|
|CSRF|SameSite=Strict Cookie + カスタムヘッダ検証|
|XSS|Vue.jsのテンプレートによるデフォルトエスケープ + CSP設定|
|SQLインジェクション|Supabaseクライアントによるパラメータ化クエリ|

### NFR-002: パフォーマンス

|項目|内容|
|-|-|
|レスポンスタイム|API応答 95パーセンタイルで500ms以内|
|同時接続数|100ユーザー以上|
|ページロード|初回ロード3秒以内（LTE環境）|

### NFR-003: 可用性

|項目|内容|
|-|-|
|稼働率|99.5%以上（月間ダウンタイム約3.6時間以内）|
|自動スケーリング|Cloud Runのオートスケーリングを活用|
|コールドスタート|最小インスタンス1を設定し、常時応答可能|

### NFR-004: 保守性

|項目|内容|
|-|-|
|コード規約|Rust: clippy準拠 / TypeScript: ESLint + Prettier|
|ログ|構造化ログ（JSON形式）で Cloud Logging へ出力|
|エラーハンドリング|Rustの Result 型による明示的なエラー伝播|

 ---

## 5 . 制約事項

* バックエンドは Rust で実装する
* フロントエンドは Vue 3 + TypeScript + Bootstrap で実装する
* データベースは Supabase（PostgreSQL）を使用する
* デプロイ環境は Google Cloud Run とする
* Supabase Auth は使用せず、独自認証を構築する
* DBテーブル名・カラム名は指定の命名規則に厳密に従う

 ---

## 6 . 用語定義

|用語|定義|
|-|-|
|資格マスタ|全ユーザー共通の資格名称データベース|
|所持資格|ユーザーが既に取得済みの資格|
|目標|ユーザーが今後取得を目指す資格と期限|
|グッドマーク|目標達成実績のあるユーザーに付与されるバッジ|
|お気に入り|他ユーザーをブックマークする機能|

 ---

 ---

# 第2部 基本設計書

## 1 . システム構成

### 1.1 全体アーキテクチャ

```
┌──────────────────────────────────────────────────────────────────┐
│                        Google Cloud Run                          │
│                                                                  │
│  ┌─────────────────────┐    ┌──────────────────────────────────┐ │
│  │   フロントエンド      │    │       バックエンド（API）          │ │
│  │   Vue 3 + TS         │───▶│       Rust (Actix-web)          │ │
│  │   Bootstrap          │◀───│                                  │ │
│  │   (静的ファイル配信)   │    │   ┌──────────────────────────┐  │ │
│  └─────────────────────┘    │   │  セッション管理           │  │ │
│                              │   │  認証ロジック             │  │ │
│                              │   │  ビジネスロジック          │  │ │
│                              │   └──────────────────────────┘  │ │
│                              └──────────┬───────────────────────┘ │
└─────────────────────────────────────────┼────────────────────────┘
                                          │ HTTPS
                                          ▼
                              ┌──────────────────────┐
                              │     Supabase          │
                              │   (PostgreSQL)        │
                              │   REST API経由        │
                              └──────────────────────┘
```

### 1.2 技術選定理由

|技術|選定理由|
|-|-|
|Rust (Actix-web)|型安全性・メモリ安全性・高パフォーマンスを兼備。Cloud Run上での低リソース消費に有利|
|Vue 3 + TypeScript|Composition APIによるロジック再利用性。TypeScriptによる型安全なフロントエンド開発|
|Bootstrap|実績あるレスポンシブフレームワーク。カスタマイズ性が高く独自デザインの基盤に適する|
|Supabase|PostgreSQLベースのBaaS。REST APIを直接利用することでRustからのアクセスが容易|
|Google Cloud Run|コンテナベースのサーバーレス。オートスケーリングとHTTPS自動対応|
|Argon2id|OWASP推奨のパスワードハッシュアルゴリズム。サイドチャネル攻撃耐性|
|HMAC-SHA256|メールアドレスの検索可能なハッシュ化に最適。サーバー管理キーによる安全性確保|

 ---

## 2 . 画面設計

### 2.1 画面一覧

|画面ID|画面名|パス|認証|概要|
|-|-|-|-|-|
|SCR-001|サインアップ|/signup|不要|新規ユーザー登録画面|
|SCR-002|サインイン|/signin|不要|ログイン画面|
|SCR-003|ダッシュボード|/dashboard|必要|所持資格・目標の概要表示|
|SCR-004|所持資格一覧|/certifications|必要|所持資格のCRUD|
|SCR-005|目標一覧|/goals|必要|目標のCRUD|
|SCR-006|ユーザー一覧|/community|必要|他ユーザーの状況表示・お気に入り|
|SCR-007|ユーザー詳細|/community/:id|必要|特定ユーザーの資格・目標詳細|

### 2.2 画面遷移図

```
 [サインアップ] ──登録完了──▶  [サインイン]
                               │
                         認証成功
                               ▼
                          [ダッシュボード]
                          │    │    │
              ┌───────────┘    │    └───────────┐
              ▼                ▼                ▼
       [所持資格一覧]      [目標一覧]       [ユーザー一覧]
                                               │
                                               ▼
                                         [ユーザー詳細]
```

### 2.3 各画面レイアウト方針

#### SCR-001 / SCR-002: サインアップ / サインイン

* 画面中央にフォームカードを1つだけ配置する。背景はニュートラルグレー(#F5F5F5)
* 主役はフォーム。ロゴはフォーム上部に小さく配置し、過剰なブランディングは避ける
* フォームフィールドは最低限（ユーザー名・メールアドレス・パスワード / メールアドレス・パスワード）
* ボタンはプライマリカラー（#1A73E8）の1色。それ以外のアクション色は使わない
* バリデーションエラーは該当フィールド直下に赤テキスト(#D32F2F)で即時表示

#### SCR-003: ダッシュボード

* 上部に「所持資格数」「達成目標数」「進行中目標数」のサマリーを3カラムで配置
* サマリーは数値を大きく、ラベルを小さく表示し、数値が視線の起点になる設計
* 下部は左右2カラム構成。左に直近の所持資格（最新3件）、右に進行中の目標（最新3件）
* 「もっと見る」リンクで各一覧画面へ遷移

#### SCR-004: 所持資格一覧

* 上部に「+ 新規登録」ボタン（プライマリカラー）を右寄せ配置
* 資格カードはリスト形式で縦に並べる。各カードに資格名（大きく太字）、取得日（サブテキスト）、編集・削除ボタン
* 削除は確認モーダルを挟む
* 新規登録・編集はモーダルダイアログで処理する。画面遷移を最小限にする
* 0件時は空状態メッセージ「まだ資格が登録されていません。」を中央に表示

#### SCR-005: 目標一覧

* ステータスごとにセクション分け（学習中 / 受験予定 / 達成 / 中断）
* 各セクション見出しの右にバッジで件数を表示
* 目標カードには資格名・目標日・残り日数・ステータスバッジ・メモ（1行クランプ）を表示
* 目標日超過の場合、残り日数部分を警告色（#E65100）に変更
* 新規追加・編集はモーダルダイアログで処理

#### SCR-006: ユーザー一覧（コミュニティ）

* 「崩一覧」を以下の手法で実現する:

  * ユーザーカードの高さを情報量に応じて可変にする
  * カード内の情報配置にアシンメトリーを持たせる（左寄せの名前、右寄せの数値）
  * カード間の余白を完全均等にせず、お気に入りブロックと一般ブロックの間にセクション区切り線を入れる
* お気に入りユーザーは上部にまとめて表示。☆マークは塗りつぶし状態(★)で表示
* 一般ユーザーは下部に表示。☆マークは線のみ(☆)で表示
* グッドマーク: 目標を1件以上達成したユーザーに緑(#2E7D32)のチェックバッジを表示
* ページネーション（20件/ページ）

#### SCR-007: ユーザー詳細

* ユーザー名とグッドマークを画面上部に表示
* 所持資格リスト・目標リストをタブ切替で表示
* 閲覧のみ（編集不可）

### 2.4 共通UIコンポーネント

|コンポーネント|用途|
|-|-|
|AppHeader|ナビゲーションバー。ロゴ・メニュー・ユーザー名・サインアウトボタン|
|AppFooter|コピーライト表示（最小限）|
|ConfirmModal|削除確認等の汎用確認モーダル|
|FormModal|登録・編集用のフォームモーダル|
|CertAutocomplete|資格名入力補完コンポーネント|
|StatusBadge|ステータス表示用バッジ（学習中=ブルー/#1565C0、受験予定=アンバー/#F57F17、達成=グリーン/#2E7D32、中断=グレー/#616161）|
|GoodMark|目標達成バッジ（緑チェック）|
|FavoriteStar|お気に入りトグルボタン（☆/★）|
|EmptyState|データ0件時の空状態表示|
|Pagination|ページネーションコントロール|
|ToastNotification|操作完了・エラー通知のトースト|

 ---

## 3 . カラー設計

### 3.1 カラーパレット

|役割|色名|カラーコード|用途|
|-|-|-|-|
|ブランドカラー|ディープブルー|#1A73E8|ロゴ・ヘッダー・主要ボタン|
|操作色|プライマリブルー|#1A73E8|CTA・リンク・プライマリボタン|
|操作色（ホバー）|ダークブルー|#1557B0|ボタンホバー時|
|状態色（成功）|グリーン|#2E7D32|達成・グッドマーク・成功通知|
|状態色（警告）|アンバー|#F57F17|目標期限接近|
|状態色（危険）|ディープオレンジ|#E65100|目標期限超過|
|状態色（エラー）|レッド|#D32F2F|バリデーションエラー・削除ボタン|
|状態色（情報）|ライトブルー|#1565C0|学習中ステータス|
|中立色（テキスト）|ダークグレー|#212121|見出し・本文|
|中立色（サブテキスト）|ミディアムグレー|#757575|補足テキスト・ラベル|
|中立色（ボーダー）|ライトグレー|#E0E0E0|カード枠・区切り線|
|中立色（背景）|オフホワイト|#F5F5F5|ページ背景|
|中立色（カード背景）|ホワイト|#FFFFFF|カード・モーダル背景|
|中立色（無効）|ペールグレー|#9E9E9E|無効状態のテキスト・ボタン|
|お気に入り（活性）|ゴールド|#F9A825|★お気に入り済み|
|お気に入り（非活性）|グレー|#BDBDBD|☆未お気に入り|

### 3.2 色設計ルール

* 1つの色に複数の意味を持たせない
* ブルー系は操作色として統一。状態色（成功・警告・エラー）と混同しない
* お気に入りのゴールドは他のどの状態色とも被らない固有色
* 紫・グラデーションは使用しない

 ---

## 4 . API設計

### 4.1 API一覧

|メソッド|エンドポイント|概要|認証|
|-|-|-|-|
|POST|/api/auth/signup|サインアップ|不要|
|POST|/api/auth/signin|サインイン|不要|
|POST|/api/auth/signout|サインアウト|必要|
|GET|/api/auth/me|自身のユーザー情報取得|必要|
|GET|/api/certifications|所持資格一覧取得|必要|
|POST|/api/certifications|所持資格登録|必要|
|PUT|/api/certifications/:id|所持資格更新|必要|
|DELETE|/api/certifications/:id|所持資格削除|必要|
|GET|/api/goals|目標一覧取得|必要|
|POST|/api/goals|目標登録|必要|
|PUT|/api/goals/:id|目標更新|必要|
|DELETE|/api/goals/:id|目標削除|必要|
|GET|/api/master/certifications|資格マスタ検索（クエリパラメータ: q）|必要|
|GET|/api/community/users|ユーザー一覧取得（ページネーション付き）|必要|
|GET|/api/community/users/:id|ユーザー詳細取得|必要|
|POST|/api/favorites/:userId|お気に入り登録|必要|
|DELETE|/api/favorites/:userId|お気に入り解除|必要|
|GET|/api/favorites|お気に入り一覧取得|必要|

### 4.2 API詳細仕様

#### POST /api/auth/signup

```
Request:
{
  "username": "string (3〜30文字)",
  "email": "string (メールアドレス形式)",
  "password": "string (8文字以上、英数字記号混合)"
}

Response (201):
{
  "message": "アカウントが作成されました"
}

Error (409):
{
  "error": "このメールアドレスは既に登録されています"
}

Error (400):
{
  "error": "バリデーションエラー",
  "details":  [
    { "field": "password", "message": "パスワードは8文字以上で入力してください" }
  ]
}
```

#### POST /api/auth/signin

```
Request:
{
  "email": "string",
  "password": "string"
}

Response (200):
Set-Cookie: session _token=<token>; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=31536000
{
  "user": {
    "id": "uuid",
    "username": "string"
  }
}

Error (401):
{
  "error": "メールアドレスまたはパスワードが正しくありません"
}
```

#### POST /api/auth/signout

```
Response (200):
Set-Cookie: session _token=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0
{
  "message": "サインアウトしました"
}
```

#### GET /api/auth/me

```
Response (200):
{
  "id": "uuid",
  "username": "string",
  "created _at": "ISO8601"
}
```

#### GET /api/certifications

```
Response (200):
{
  "certifications":  [
    {
      "id": "uuid",
      "certification _name": "string",
      "master _id": "uuid",
      "acquired _date": "YYYY-MM-DD",
      "created _at": "ISO8601"
    }
  ]
}
```

#### POST /api/certifications

```
Request:
{
  "certification _name": "string",
  "master _id": "uuid | null",
  "acquired _date": "YYYY-MM-DD"
}

Response (201):
{
  "id": "uuid",
  "certification _name": "string",
  "master _id": "uuid",
  "acquired _date": "YYYY-MM-DD"
}
```

#### PUT /api/certifications/:id

```
Request:
{
  "certification _name": "string",
  "acquired _date": "YYYY-MM-DD"
}

Response (200):
{
  "id": "uuid",
  "certification _name": "string",
  "acquired _date": "YYYY-MM-DD"
}
```

#### DELETE /api/certifications/:id

```
Response (204): No Content
```

#### GET /api/goals

```
Response (200):
{
  "goals":  [
    {
      "id": "uuid",
      "certification _name": "string",
      "master _id": "uuid",
      "target _date": "YYYY-MM-DD",
      "status": "studying | scheduled | achieved | suspended",
      "memo": "string | null",
      "created _at": "ISO8601"
    }
  ]
}
```

#### POST /api/goals

```
Request:
{
  "certification _name": "string",
  "master _id": "uuid | null",
  "target _date": "YYYY-MM-DD",
  "status": "studying",
  "memo": "string | null"
}

Response (201):
{
  "id": "uuid",
  ...
}
```

#### PUT /api/goals/:id

```
Request:
{
  "target _date": "YYYY-MM-DD",
  "status": "studying | scheduled | achieved | suspended",
  "memo": "string | null"
}

Response (200): 更新後のGoalオブジェクト
```

#### DELETE /api/goals/:id

```
Response (204): No Content
```

#### GET /api/master/certifications?q=キーワード

```
Response (200):
{
  "certifications":  [
    {
      "id": "uuid",
      "name": "string",
      "category": "string"
    }
  ]
}
```

#### GET /api/community/users?page=1 &per _page=20

```
Response (200):
{
  "users":  [
    {
      "id": "uuid",
      "username": "string",
      "certification _count": 5,
      "goal _count": 3,
      "achieved _count": 2,
      "has _good _mark": true,
      "is _favorite": false
    }
  ],
  "total": 150,
  "page": 1,
  "per _page": 20
}
```

#### GET /api/community/users/:id

```
Response (200):
{
  "id": "uuid",
  "username": "string",
  "has _good _mark": true,
  "certifications":  [...],
  "goals":  [...]
}
```

#### POST /api/favorites/:userId

```
Response (201):
{
  "message": "お気に入りに追加しました"
}
```

#### DELETE /api/favorites/:userId

```
Response (204): No Content
```

 ---

## 5 . データベース設計

### 5.1 ER図

```
TBL _USER ─────┬───< TBL _HOLDING >───── TBL _MASTER
              │                              │
              ├───< TBL _GOAL >───────────────┘
              │
              ├───< TBL _FAVORITE (favui) >
              │
              ├───< TBL _FAVORITE (favti) >
              │
              └───< TBL _SESSION >
```

### 5.2 テーブル定義

#### TBL _USER（ユーザー）

|カラム名|型|制約|説明|
|-|-|-|-|
|useid|UUID|PK, NOT NULL, DEFAULT gen _random _uuid()|ユーザーID|
|usenm|VARCHAR(30)|NOT NULL|ユーザー名（表示名）|
|useml|VARCHAR(128)|NOT NULL, UNIQUE|メールアドレス（HMAC-SHA256ハッシュ値）|
|usepw|VARCHAR(256)|NOT NULL|パスワード（Argon2idハッシュ値）|
|useca|TIMESTAMPTZ|NOT NULL, DEFAULT now()|作成日時|
|useua|TIMESTAMPTZ|NOT NULL, DEFAULT now()|更新日時|

命名規則: `TBL _USER` → 前方3文字 `use` + 属性2文字

#### TBL _MASTER（資格マスタ）

|カラム名|型|制約|説明|
|-|-|-|-|
|masid|UUID|PK, NOT NULL, DEFAULT gen _random _uuid()|マスタID|
|masnm|VARCHAR(200)|NOT NULL|資格名称|
|masct|VARCHAR(30)|NOT NULL, DEFAULT 'その他'|カテゴリ（IT/語学/金融/医療/法律/その他）|
|masnr|VARCHAR(200)|NOT NULL, UNIQUE|正規化名称（重複判定用・小文字+空白除去）|
|masca|TIMESTAMPTZ|NOT NULL, DEFAULT now()|作成日時|

命名規則: `TBL _MASTER` → 前方3文字 `mas` + 属性2文字

#### TBL _HOLDING（所持資格）

|カラム名|型|制約|説明|
|-|-|-|-|
|holid|UUID|PK, NOT NULL, DEFAULT gen _random _uuid()|所持資格ID|
|holui|UUID|NOT NULL, FK → TBL _USER(useid) ON DELETE CASCADE|ユーザーID|
|holmi|UUID|NOT NULL, FK → TBL _MASTER(masid)|マスタID|
|holdt|DATE|NULL|取得日|
|holca|TIMESTAMPTZ|NOT NULL, DEFAULT now()|作成日時|
|holua|TIMESTAMPTZ|NOT NULL, DEFAULT now()|更新日時|

命名規則: `TBL _HOLDING` → 前方3文字 `hol` + 属性2文字

#### TBL _GOAL（目標）

|カラム名|型|制約|説明|
|-|-|-|-|
|goaid|UUID|PK, NOT NULL, DEFAULT gen _random _uuid()|目標ID|
|goaui|UUID|NOT NULL, FK → TBL _USER(useid) ON DELETE CASCADE|ユーザーID|
|goami|UUID|NOT NULL, FK → TBL _MASTER(masid)|マスタID|
|goatd|DATE|NOT NULL|目標日|
|goast|VARCHAR(20)|NOT NULL, DEFAULT 'studying'|ステータス|
|goamm|TEXT|NULL|メモ|
|goaca|TIMESTAMPTZ|NOT NULL, DEFAULT now()|作成日時|
|goaua|TIMESTAMPTZ|NOT NULL, DEFAULT now()|更新日時|

命名規則: `TBL _GOAL` → 前方3文字 `goa` + 属性2文字

ステータス値: `studying`（学習中）/ `scheduled`（受験予定）/ `achieved`（達成）/ `suspended`（中断）

#### TBL _FAVORITE（お気に入り）

|カラム名|型|制約|説明|
|-|-|-|-|
|favid|UUID|PK, NOT NULL, DEFAULT gen _random _uuid()|お気に入りID|
|favui|UUID|NOT NULL, FK → TBL _USER(useid) ON DELETE CASCADE|お気に入り登録者のユーザーID|
|favti|UUID|NOT NULL, FK → TBL _USER(useid) ON DELETE CASCADE|お気に入り対象のユーザーID|
|favca|TIMESTAMPTZ|NOT NULL, DEFAULT now()|作成日時|

命名規則: `TBL _FAVORITE` → 前方3文字 `fav` + 属性2文字
制約: UNIQUE(favui, favti) — 同一ペアの重複登録防止
制約: CHECK(favui != favti) — 自分自身をお気に入り登録不可

#### TBL _SESSION（セッション）

|カラム名|型|制約|説明|
|-|-|-|-|
|sesid|UUID|PK, NOT NULL, DEFAULT gen _random _uuid()|セッションID|
|sesui|UUID|NOT NULL, FK → TBL _USER(useid) ON DELETE CASCADE|ユーザーID|
|sestk|VARCHAR(128)|NOT NULL, UNIQUE|セッショントークン（256bit乱数のBase64）|
|sesea|TIMESTAMPTZ|NOT NULL|有効期限（作成時から1年後）|
|sesca|TIMESTAMPTZ|NOT NULL, DEFAULT now()|作成日時|

命名規則: `TBL _SESSION` → 前方3文字 `ses` + 属性2文字

### 5.3 インデックス設計

|テーブル|インデックス名|対象カラム|種類|理由|
|-|-|-|-|-|
|TBL _USER|idx _user _email|useml|UNIQUE|サインイン時のメール検索|
|TBL _HOLDING|idx _holding _user|holui|B-tree|ユーザーごとの所持資格取得|
|TBL _GOAL|idx _goal _user|goaui|B-tree|ユーザーごとの目標取得|
|TBL _GOAL|idx _goal _status|goast|B-tree|ステータスでのフィルタリング|
|TBL _FAVORITE|idx _fav _user|favui|B-tree|ユーザーのお気に入り一覧取得|
|TBL _FAVORITE|idx _fav _unique|favui, favti|UNIQUE|重複登録防止|
|TBL _SESSION|idx _session _token|sestk|UNIQUE|トークンによるセッション検索|
|TBL _SESSION|idx _session _expiry|sesea|B-tree|期限切れセッションの定期削除|
|TBL _MASTER|idx _master _norm|masnr|UNIQUE|正規化名称による重複チェック|
|TBL _MASTER|idx _master _name _trgm|masnm|GIN (pg _trgm)|あいまい検索・入力補完|

### 5.4 DDL

```sql
-- 拡張機能の有効化
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg _trgm";

-- TBL _USER
CREATE TABLE "TBL _USER" (
    useid UUID PRIMARY KEY DEFAULT gen _random _uuid(),
    usenm VARCHAR(30) NOT NULL,
    useml VARCHAR(128) NOT NULL UNIQUE,
    usepw VARCHAR(256) NOT NULL,
    useca TIMESTAMPTZ NOT NULL DEFAULT now(),
    useua TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TBL _MASTER
CREATE TABLE "TBL _MASTER" (
    masid UUID PRIMARY KEY DEFAULT gen _random _uuid(),
    masnm VARCHAR(200) NOT NULL,
    masct VARCHAR(30) NOT NULL DEFAULT 'その他',
    masnr VARCHAR(200) NOT NULL UNIQUE,
    masca TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TBL _HOLDING
CREATE TABLE "TBL _HOLDING" (
    holid UUID PRIMARY KEY DEFAULT gen _random _uuid(),
    holui UUID NOT NULL REFERENCES "TBL _USER"(useid) ON DELETE CASCADE,
    holmi UUID NOT NULL REFERENCES "TBL _MASTER"(masid),
    holdt DATE,
    holca TIMESTAMPTZ NOT NULL DEFAULT now(),
    holua TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TBL _GOAL
CREATE TABLE "TBL _GOAL" (
    goaid UUID PRIMARY KEY DEFAULT gen _random _uuid(),
    goaui UUID NOT NULL REFERENCES "TBL _USER"(useid) ON DELETE CASCADE,
    goami UUID NOT NULL REFERENCES "TBL _MASTER"(masid),
    goatd DATE NOT NULL,
    goast VARCHAR(20) NOT NULL DEFAULT 'studying'
        CHECK (goast IN ('studying', 'scheduled', 'achieved', 'suspended')),
    goamm TEXT,
    goaca TIMESTAMPTZ NOT NULL DEFAULT now(),
    goaua TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TBL _FAVORITE
CREATE TABLE "TBL _FAVORITE" (
    favid UUID PRIMARY KEY DEFAULT gen _random _uuid(),
    favui UUID NOT NULL REFERENCES "TBL _USER"(useid) ON DELETE CASCADE,
    favti UUID NOT NULL REFERENCES "TBL _USER"(useid) ON DELETE CASCADE,
    favca TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (favui, favti),
    CHECK (favui != favti)
);

-- TBL _SESSION
CREATE TABLE "TBL _SESSION" (
    sesid UUID PRIMARY KEY DEFAULT gen _random _uuid(),
    sesui UUID NOT NULL REFERENCES "TBL _USER"(useid) ON DELETE CASCADE,
    sestk VARCHAR(128) NOT NULL UNIQUE,
    sesea TIMESTAMPTZ NOT NULL,
    sesca TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- インデックス
CREATE INDEX idx _holding _user ON "TBL _HOLDING"(holui);
CREATE INDEX idx _goal _user ON "TBL _GOAL"(goaui);
CREATE INDEX idx _goal _status ON "TBL _GOAL"(goast);
CREATE INDEX idx _fav _user ON "TBL _FAVORITE"(favui);
CREATE INDEX idx _session _expiry ON "TBL _SESSION"(sesea);
CREATE INDEX idx _master _name _trgm ON "TBL _MASTER" USING GIN (masnm gin _trgm _ops);

-- 更新日時の自動更新トリガー
CREATE OR REPLACE FUNCTION update _updated _at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.useua = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- TBL _USERの更新トリガー
CREATE TRIGGER trg _user _updated
    BEFORE UPDATE ON "TBL _USER"
    FOR EACH ROW EXECUTE FUNCTION update _updated _at();

-- TBL _HOLDINGの更新トリガー（holua用）
CREATE OR REPLACE FUNCTION update _holding _updated _at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.holua = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg _holding _updated
    BEFORE UPDATE ON "TBL _HOLDING"
    FOR EACH ROW EXECUTE FUNCTION update _holding _updated _at();

-- TBL _GOALの更新トリガー（goaua用）
CREATE OR REPLACE FUNCTION update _goal _updated _at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.goaua = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg _goal _updated
    BEFORE UPDATE ON "TBL _GOAL"
    FOR EACH ROW EXECUTE FUNCTION update _goal _updated _at();
```

 ---

## 6 . 認証・セキュリティ設計

### 6.1 パスワードハッシュ化

**アルゴリズム**: Argon2id
**選定理由**: OWASP推奨。メモリ硬化型でGPU/ASIC攻撃に強い。サイドチャネル攻撃耐性を持つidバリアントを採用。

**パラメータ**:

|項目|値|理由|
|-|-|-|
|メモリコスト|65536 KiB (64MB)|OWASP推奨の最小値|
|反復回数|3|メモリコストとのバランス|
|並列度|4|一般的なサーバーCPUコア数|
|ソルト長|16バイト|十分なエントロピー|
|ハッシュ長|32バイト|標準的な長さ|

**Rustライブラリ**: `argon2` crate（RustCryptoプロジェクト）

### 6.2 メールアドレスハッシュ化

**アルゴリズム**: HMAC-SHA256
**選定理由**: サインイン時にハッシュ化されたメールで検索する必要があるため、同一入力に対して常に同一ハッシュを生成する決定論的なハッシュが必要。HMAC-SHA256はサーバー管理のシークレットキーを使用することで、レインボーテーブル攻撃を防止する。

**設計**:

* シークレットキーは環境変数 `EMAIL _HMAC _SECRET` に格納する
* キー長は32バイト以上とする
* ハッシュ値はHex文字列（64文字）としてDBに保存する

**Rustライブラリ**: `hmac` + `sha2` crate（RustCryptoプロジェクト）

### 6.3 セッション管理

**トークン生成**: `rand` crateで256bitの暗号学的乱数を生成し、Base64エンコードして使用する

**セッションフロー**:

1. サインイン成功時にセッショントークンを生成
2. TBL _SESSIONにトークン・ユーザーID・有効期限（1年後）を保存
3. レスポンスのSet-Cookieヘッダーにトークンを設定（HttpOnly, Secure, SameSite=Strict）
4. 以降のリクエストでCookieからトークンを読み取り、TBL _SESSIONで照合
5. サインアウト時にTBL _SESSIONからレコード削除 + Cookieクリア

**セキュリティ属性**:

|Cookie属性|設定|理由|
|-|-|-|
|HttpOnly|true|JavaScriptからのアクセスを防止（XSS対策）|
|Secure|true|HTTPS通信でのみ送信|
|SameSite|Strict|他サイトからのリクエストでCookie送信を完全ブロック（CSRF対策）|
|Path|/|アプリケーション全体で有効|
|Max-Age|31536000 (1年)|永続セッション（要件準拠）|

### 6.4 入力バリデーション

|フィールド|ルール|
|-|-|
|ユーザー名|3〜30文字、英数字・日本語・アンダースコア|
|メールアドレス|RFC 5322準拠の形式チェック|
|パスワード|8文字以上、英大文字・英小文字・数字をそれぞれ1文字以上含む|
|資格名|1〜200文字|
|目標日|現在日以降の日付|
|メモ|1000文字以内|

 ---

## 7 . エラーハンドリング設計

### 7.1 エラーレスポンス形式

```json
{
  "error": "エラーメッセージ（ユーザー向け）",
  "code": "ERROR _CODE",
  "details":  []
}
```

### 7.2 エラーコード一覧

|HTTPステータス|エラーコード|説明|
|-|-|-|
|400|VALIDATION _ERROR|入力値バリデーションエラー|
|401|UNAUTHORIZED|認証が必要 / 認証失敗|
|403|FORBIDDEN|権限がない操作|
|404|NOT _FOUND|リソースが見つからない|
|409|CONFLICT|重複登録（メールアドレスなど）|
|500|INTERNAL _ERROR|サーバー内部エラー|

 ---

 ---

# 第3部 詳細設計書

## 1 . バックエンド詳細設計

### 1.1 プロジェクト構成

```
backend/
├── Cargo.toml
├── Dockerfile
├── src/
│   ├── main.rs                  # エントリーポイント・サーバー起動
│   ├── config.rs                # 環境変数・設定管理
│   ├── db.rs                    # Supabase HTTPクライアント初期化
│   ├── errors.rs                # エラー型定義・レスポンス変換
│   ├── middleware/
│   │   └── auth.rs              # 認証ミドルウェア（セッション検証）
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs              # User構造体
│   │   ├── certification.rs     # Holding構造体
│   │   ├── goal.rs              # Goal構造体
│   │   ├── master.rs            # Master構造体
│   │   ├── favorite.rs          # Favorite構造体
│   │   └── session.rs           # Session構造体
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth.rs              # 認証ハンドラ（signup/signin/signout/me）
│   │   ├── certification.rs     # 所持資格ハンドラ（CRUD）
│   │   ├── goal.rs              # 目標ハンドラ（CRUD）
│   │   ├── master.rs            # 資格マスタハンドラ（検索）
│   │   ├── community.rs         # コミュニティハンドラ（ユーザー一覧・詳細）
│   │   └── favorite.rs          # お気に入りハンドラ
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth _service.rs      # 認証ビジネスロジック
│   │   ├── cert _service.rs      # 所持資格ビジネスロジック
│   │   ├── goal _service.rs      # 目標ビジネスロジック
│   │   ├── master _service.rs    # マスタビジネスロジック
│   │   ├── community _service.rs # コミュニティビジネスロジック
│   │   └── favorite _service.rs  # お気に入りビジネスロジック
│   └── utils/
│       ├── mod.rs
│       ├── hash.rs              # Argon2id・HMAC-SHA256ユーティリティ
│       └── token.rs             # セッショントークン生成ユーティリティ
```

### 1.2 主要Crateと依存関係

|Crate|バージョン|用途|
|-|-|-|
|actix-web|4.x|HTTPサーバーフレームワーク|
|actix-cors|0.7.x|CORS制御|
|serde / serde _json|1.x|JSON シリアライゼーション|
|reqwest|0.12.x|Supabase REST API用HTTPクライアント|
|argon2|0.5.x|パスワードハッシュ（Argon2id）|
|hmac / sha2|0.12.x / 0.10.x|メールアドレスHMAC-SHA256ハッシュ|
|rand|0.8.x|セッショントークン用暗号学的乱数生成|
|base64|0.22.x|トークンBase64エンコード|
|uuid|1.x|UUID生成|
|chrono|0.4.x|日時操作|
|dotenvy|0.15.x|環境変数ファイル読み込み|
|log / env _logger|0.4.x / 0.11.x|ロギング|
|validator|0.18.x|入力バリデーション|
|tokio|1.x|非同期ランタイム|

### 1.3 設定管理（config.rs）

```rust
pub struct Config {
    pub supabase _url: String,        // Supabase REST APIのURL
    pub supabase _key: String,        // Supabase anon key
    pub email _hmac _secret: String,   // メールアドレスHMAC用シークレット
    pub server _port: u16,            // サーバーポート（デフォルト: 8080）
    pub cors _origin: String,         // 許可するフロントエンドオリジン
}
```

全て環境変数から読み込む。`.env` ファイルによるローカル開発もサポート。

### 1.4 Supabase HTTPクライアント（db.rs）

SupabaseのREST API（PostgREST）をHTTPクライアント（reqwest）で直接呼び出す設計。

```rust
pub struct SupabaseClient {
    client: reqwest::Client,
    base _url: String,
    api _key: String,
}

impl SupabaseClient {
    // SELECT: GET /rest/v1/{table}?{filters}
    pub async fn select( &self, table:  &str, query:  &str) -> Result<Value>
    
    // INSERT: POST /rest/v1/{table}
    pub async fn insert( &self, table:  &str, body:  &Value) -> Result<Value>
    
    // UPDATE: PATCH /rest/v1/{table}?{filters}
    pub async fn update( &self, table:  &str, query:  &str, body:  &Value) -> Result<Value>
    
    // DELETE: DELETE /rest/v1/{table}?{filters}
    pub async fn delete( &self, table:  &str, query:  &str) -> Result<()>
    
    // RPC: POST /rest/v1/rpc/{function _name}
    pub async fn rpc( &self, function:  &str, body:  &Value) -> Result<Value>
}
```

HTTPヘッダーには `apikey` と `Authorization: Bearer {key}` を設定。
`Prefer: return=representation` を指定し、INSERT/UPDATEの戻り値として挿入/更新後のレコードを取得する。

### 1.5 認証ミドルウェア（middleware/auth.rs）

リクエストごとにCookieからセッショントークンを取り出し、TBL _SESSIONを検索して有効性を検証する。

**処理フロー**:

1. Cookieヘッダーから `session _token` を取得
2. トークンが存在しない場合は 401 を返す
3. TBL _SESSION で `sestk = トークン` かつ `sesea > now()` のレコードを検索
4. レコードが存在しない場合は 401 を返す
5. レコードのユーザーID（sesui）をリクエストエクステンションに格納
6. ハンドラからユーザーIDを参照可能にする

### 1.6 ハッシュユーティリティ（utils/hash.rs）

```rust
/// パスワードをArgon2idでハッシュ化する
/// 戻り値はPHC文字列形式（ソルト内包）
pub fn hash _password(password:  &str) -> Result<String>

/// パスワードの検証（PHC文字列と平文を照合）
pub fn verify _password(hash:  &str, password:  &str) -> Result<bool>

/// メールアドレスをHMAC-SHA256でハッシュ化する
/// 入力は正規化（小文字化・前後空白除去）してからハッシュ化
pub fn hash _email(email:  &str, secret:  &str) -> String
```

### 1.7 認証ハンドラ詳細（handlers/auth.rs）

#### signup処理フロー

1. リクエストボディのバリデーション（ユーザー名・メール・パスワード）
2. メールアドレスを正規化（小文字化 + 前後空白除去）
3. HMAC-SHA256でメールアドレスをハッシュ化
4. TBL _USERで既存メールハッシュの存在チェック → 重複時409エラー
5. Argon2idでパスワードをハッシュ化
6. TBL _USERにレコード挿入（useid自動生成、usenm、useml=ハッシュ、usepw=ハッシュ）
7. 201レスポンスを返す

#### signin処理フロー

1. リクエストボディのバリデーション
2. メールアドレスを正規化 → HMAC-SHA256でハッシュ化
3. TBL _USERで `useml = ハッシュ値` のレコードを検索
4. レコードが存在しない場合は401エラー（メッセージは曖昧にする）
5. Argon2idでパスワードを検証
6. 検証失敗時は401エラー
7. 256bitの暗号学的乱数を生成 → Base64エンコード → セッショントークン
8. TBL _SESSIONにトークンを保存（有効期限=1年後）
9. Set-Cookieヘッダーにトークンを設定
10. ユーザー情報（id, username）をレスポンス

#### signout処理フロー

1. 認証ミドルウェアでセッション検証（トークン取得）
2. TBL _SESSIONから該当トークンのレコードを削除
3. Set-Cookieで有効期限0のCookieを設定（Cookie削除）
4. 200レスポンスを返す

### 1.8 所持資格ハンドラ詳細（handlers/certification.rs）

#### 一覧取得 (GET /api/certifications)

1. 認証ミドルウェアからユーザーIDを取得
2. TBL _HOLDINGで `holui = ユーザーID` のレコードを取得（TBL _MASTERとJOIN、masid, masnmも取得）
3. レスポンスとして返す

#### 登録 (POST /api/certifications)

1. リクエストボディのバリデーション
2. master _idが未指定の場合:
a. certification _nameを正規化
b. TBL _MASTERで正規化名が一致するレコードを検索
c. 存在しない場合はTBL _MASTERに新規レコードを挿入
d. masidを取得
3. TBL _HOLDINGに新規レコードを挿入
4. 201レスポンスを返す

#### 更新 (PUT /api/certifications/:id)

1. パスパラメータからIDを取得
2. TBL _HOLDINGで `holid = ID AND holui = ユーザーID` のレコードを確認（所有権検証）
3. 存在しない場合は404エラー
4. TBL _HOLDINGを更新
5. 200レスポンスを返す

#### 削除 (DELETE /api/certifications/:id)

1. パスパラメータからIDを取得
2. TBL _HOLDINGで `holid = ID AND holui = ユーザーID` のレコードを確認
3. 存在しない場合は404エラー
4. TBL _HOLDINGからレコードを削除
5. 204レスポンスを返す

### 1.9 目標ハンドラ詳細（handlers/goal.rs）

#### 一覧取得 (GET /api/goals)

1. 認証ミドルウェアからユーザーIDを取得
2. TBL _GOALで `goaui = ユーザーID` のレコードを取得（TBL _MASTERとJOIN）
3. ステータスごとにグルーピングしてレスポンス

#### 登録 (POST /api/goals)

1. リクエストボディのバリデーション（target _dateが未来日であること等）
2. master _idが未指定の場合はマスタ自動登録（所持資格と同様のロジック）
3. TBL _GOALに新規レコードを挿入（デフォルトステータス: studying）
4. 201レスポンスを返す

#### 更新 (PUT /api/goals/:id)

1. 所有権検証（goaui = ユーザーID）
2. ステータスが `achieved` に変更された場合:
a. レスポンスに `promote _to _holding: true` フラグを含める
b. フロントエンドで所持資格への変換確認を行う
3. TBL _GOALを更新

#### 削除 (DELETE /api/goals/:id)

1. 所有権検証
2. TBL _GOALからレコードを削除
3. 204レスポンスを返す

### 1.10 コミュニティハンドラ詳細（handlers/community.rs）

#### ユーザー一覧 (GET /api/community/users)

1. クエリパラメータからページ番号・件数を取得（デフォルト: page=1, per _page=20）
2. Supabase RPCで以下の集計クエリを実行:

   * TBL _USERの全ユーザーを取得
   * TBL _HOLDINGの件数をCOUNTでJOIN
   * TBL _GOALの件数をCOUNTでJOIN
   * TBL _GOALでgoast='achieved'の件数をCOUNTでJOIN → 1件以上ならhas _good _mark=true
   * TBL _FAVORITEで自ユーザーのお気に入りかどうかを判定 → is _favorite
3. ソート順: is _favorite=true → 上位、その後はcertification _count降順
4. ページネーション適用
5. レスポンスを返す

**SupabaseのRPC関数定義（DDL）**:

```sql
CREATE OR REPLACE FUNCTION get _community _users(
    p _user _id UUID,
    p _limit INT DEFAULT 20,
    p _offset INT DEFAULT 0
)
RETURNS TABLE (
    user _id UUID,
    username VARCHAR,
    certification _count BIGINT,
    goal _count BIGINT,
    achieved _count BIGINT,
    has _good _mark BOOLEAN,
    is _favorite BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        u.useid AS user _id,
        u.usenm AS username,
        COALESCE(h.cnt, 0) AS certification _count,
        COALESCE(g.cnt, 0) AS goal _count,
        COALESCE(a.cnt, 0) AS achieved _count,
        COALESCE(a.cnt, 0) > 0 AS has _good _mark,
        f.favid IS NOT NULL AS is _favorite
    FROM "TBL _USER" u
    LEFT JOIN (
        SELECT holui, COUNT( *) AS cnt FROM "TBL _HOLDING" GROUP BY holui
    ) h ON u.useid = h.holui
    LEFT JOIN (
        SELECT goaui, COUNT( *) AS cnt FROM "TBL _GOAL" GROUP BY goaui
    ) g ON u.useid = g.goaui
    LEFT JOIN (
        SELECT goaui, COUNT( *) AS cnt FROM "TBL _GOAL"
        WHERE goast = 'achieved' GROUP BY goaui
    ) a ON u.useid = a.goaui
    LEFT JOIN "TBL _FAVORITE" f ON f.favui = p _user _id AND f.favti = u.useid
    WHERE u.useid != p _user _id
    ORDER BY is _favorite DESC, certification _count DESC
    LIMIT p _limit OFFSET p _offset;
END;
$$ LANGUAGE plpgsql;
```

### 1.11 お気に入りハンドラ詳細（handlers/favorite.rs）

#### 登録 (POST /api/favorites/:userId)

1. パスパラメータから対象ユーザーIDを取得
2. 自分自身を登録しようとした場合は400エラー
3. TBL _FAVORITEに挿入（UNIQUE制約で重複防止）
4. 既に登録済みの場合は409エラー（またはべき等に200を返す）
5. 201レスポンスを返す

#### 解除 (DELETE /api/favorites/:userId)

1. TBL _FAVORITEで `favui = 自分 AND favti = 対象` のレコードを削除
2. 204レスポンスを返す

 ---

## 2 . フロントエンド詳細設計

### 2.1 プロジェクト構成

```
frontend/
├── package.json
├── vite.config.ts
├── tsconfig.json
├── index.html
├── public/
│   └── favicon.ico
├── src/
│   ├── main.ts                      # Vueアプリエントリーポイント
│   ├── App.vue                      # ルートコンポーネント
│   ├── router/
│   │   └── index.ts                 # Vue Router 設定・ナビゲーションガード
│   ├── stores/
│   │   ├── auth.ts                  # 認証ストア（Pinia）
│   │   ├── certification.ts         # 所持資格ストア
│   │   ├── goal.ts                  # 目標ストア
│   │   ├── community.ts             # コミュニティストア
│   │   └── favorite.ts              # お気に入りストア
│   ├── api/
│   │   ├── client.ts                # Axios インスタンス（ベースURL・インターセプター）
│   │   ├── auth.ts                  # 認証API呼び出し
│   │   ├── certification.ts         # 所持資格API呼び出し
│   │   ├── goal.ts                  # 目標API呼び出し
│   │   ├── master.ts                # 資格マスタAPI呼び出し
│   │   ├── community.ts             # コミュニティAPI呼び出し
│   │   └── favorite.ts              # お気に入りAPI呼び出し
│   ├── views/
│   │   ├── SignUpView.vue           # サインアップ画面
│   │   ├── SignInView.vue           # サインイン画面
│   │   ├── DashboardView.vue        # ダッシュボード画面
│   │   ├── CertificationView.vue    # 所持資格一覧画面
│   │   ├── GoalView.vue             # 目標一覧画面
│   │   ├── CommunityView.vue        # ユーザー一覧画面
│   │   └── UserDetailView.vue       # ユーザー詳細画面
│   ├── components/
│   │   ├── layout/
│   │   │   ├── AppHeader.vue        # 共通ヘッダー
│   │   │   └── AppFooter.vue        # 共通フッター
│   │   ├── common/
│   │   │   ├── ConfirmModal.vue     # 確認モーダル
│   │   │   ├── FormModal.vue        # フォームモーダル
│   │   │   ├── EmptyState.vue       # 空状態表示
│   │   │   ├── Pagination.vue       # ページネーション
│   │   │   └── ToastNotification.vue # トースト通知
│   │   ├── certification/
│   │   │   ├── CertCard.vue         # 所持資格カード
│   │   │   └── CertForm.vue         # 所持資格登録・編集フォーム
│   │   ├── goal/
│   │   │   ├── GoalCard.vue         # 目標カード
│   │   │   ├── GoalForm.vue         # 目標登録・編集フォーム
│   │   │   └── StatusBadge.vue      # ステータスバッジ
│   │   ├── community/
│   │   │   ├── UserCard.vue         # ユーザーカード
│   │   │   ├── GoodMark.vue         # グッドマークバッジ
│   │   │   └── FavoriteStar.vue     # お気に入りボタン
│   │   └── master/
│   │       └── CertAutocomplete.vue # 資格名入力補完
│   ├── composables/
│   │   ├── useAuth.ts               # 認証関連ロジック
│   │   ├── useToast.ts              # トースト通知ロジック
│   │   └── useDebounce.ts           # デバウンスユーティリティ
│   ├── types/
│   │   └── index.ts                 # TypeScript型定義
│   └── assets/
│       └── styles/
│           ├──  _variables.scss       # Bootstrap変数オーバーライド
│           └── main.scss             # グローバルスタイル
```

### 2.2 TypeScript型定義（types/index.ts）

```typescript
// ユーザー
export interface User {
  id: string;
  username: string;
  created _at: string;
}

// 所持資格
export interface Certification {
  id: string;
  certification _name: string;
  master _id: string;
  acquired _date: string | null;
  created _at: string;
}

// 目標
export type GoalStatus = 'studying' | 'scheduled' | 'achieved' | 'suspended';

export interface Goal {
  id: string;
  certification _name: string;
  master _id: string;
  target _date: string;
  status: GoalStatus;
  memo: string | null;
  created _at: string;
}

// 資格マスタ
export interface MasterCertification {
  id: string;
  name: string;
  category: string;
}

// コミュニティユーザー
export interface CommunityUser {
  id: string;
  username: string;
  certification _count: number;
  goal _count: number;
  achieved _count: number;
  has _good _mark: boolean;
  is _favorite: boolean;
}

// ページネーション付きレスポンス
export interface PaginatedResponse<T> {
  data: T [];
  total: number;
  page: number;
  per _page: number;
}

// フォーム入力
export interface SignUpForm {
  username: string;
  email: string;
  password: string;
}

export interface SignInForm {
  email: string;
  password: string;
}

export interface CertificationForm {
  certification _name: string;
  master _id: string | null;
  acquired _date: string;
}

export interface GoalForm {
  certification _name: string;
  master _id: string | null;
  target _date: string;
  status: GoalStatus;
  memo: string;
}
```

### 2.3 Vue Router設計（router/index.ts）

```typescript
const routes =  [
  { path: '/signup', name: 'SignUp', component: SignUpView, meta: { requiresAuth: false } },
  { path: '/signin', name: 'SignIn', component: SignInView, meta: { requiresAuth: false } },
  { path: '/dashboard', name: 'Dashboard', component: DashboardView, meta: { requiresAuth: true } },
  { path: '/certifications', name: 'Certifications', component: CertificationView, meta: { requiresAuth: true } },
  { path: '/goals', name: 'Goals', component: GoalView, meta: { requiresAuth: true } },
  { path: '/community', name: 'Community', component: CommunityView, meta: { requiresAuth: true } },
  { path: '/community/:id', name: 'UserDetail', component: UserDetailView, meta: { requiresAuth: true } },
  { path: '/', redirect: '/dashboard' },
];
```

**ナビゲーションガード**: `beforeEach`で`meta.requiresAuth`を確認し、認証ストアの状態に基づいてリダイレクト制御を行う。未認証で認証必要画面にアクセスした場合は `/signin` にリダイレクト。認証済みで `/signin` `/signup` にアクセスした場合は `/dashboard` にリダイレクト。

### 2.4 Pinia ストア詳細

#### auth.ts（認証ストア）

```
state:
  - user: User | null
  - isAuthenticated: boolean
  - loading: boolean

actions:
  - signup(form: SignUpForm): Promise<void>
  - signin(form: SignInForm): Promise<void>
  - signout(): Promise<void>
  - fetchMe(): Promise<void>  // アプリ起動時にセッション確認
```

#### certification.ts（所持資格ストア）

```
state:
  - certifications: Certification []
  - loading: boolean

actions:
  - fetchAll(): Promise<void>
  - create(form: CertificationForm): Promise<void>
  - update(id: string, form: CertificationForm): Promise<void>
  - remove(id: string): Promise<void>
```

#### goal.ts（目標ストア）

```
state:
  - goals: Goal []
  - loading: boolean

getters:
  - studyingGoals: Goal []     // ステータスが 'studying' の目標
  - scheduledGoals: Goal []    // ステータスが 'scheduled' の目標
  - achievedGoals: Goal []     // ステータスが 'achieved' の目標
  - suspendedGoals: Goal []    // ステータスが 'suspended' の目標

actions:
  - fetchAll(): Promise<void>
  - create(form: GoalForm): Promise<void>
  - update(id: string, form: Partial<GoalForm>): Promise<void>
  - remove(id: string): Promise<void>
```

### 2.5 APIクライアント設計（api/client.ts）

Axiosインスタンスを作成し、以下のインターセプターを設定:

**リクエストインターセプター**:

* `withCredentials: true` を設定（Cookie送信のため）

**レスポンスインターセプター**:

* 401レスポンス受信時に認証ストアをリセットし、`/signin` にリダイレクト
* 5xxエラー時にトースト通知を表示

### 2.6 資格名入力補完コンポーネント（CertAutocomplete.vue）

**動作仕様**:

1. テキスト入力を受け付ける
2. 入力が2文字以上になったら300msのデバウンス後にAPI呼び出し
3. GET `/api/master/certifications?q={入力値}` で候補を取得
4. ドロップダウンリストで候補を表示（最大10件）
5. 候補を選択した場合、master _idとcertification _nameを親コンポーネントに emit
6. 候補にない新規入力の場合、master _id=null としてemit
7. キーボード操作（上下矢印・Enter・Escape）に対応

### 2.7 コミュニティ画面の「崩しを入れた一覧」実装詳細

**レイアウト手法**: Bootstrapのグリッドシステム（col-12）をベースとし、CSS変数で以下を制御:

1. **カードの高さ可変**: 資格数・目標数に応じてカード内の情報量が増減するため、高さが自然に変わる。`min-height` のみ設定し、`height` は固定しない
2. **非対称な情報配置**: カード内をflex-rowで2分割。左側にユーザー名・グッドマーク（左寄せ）、右側に数値情報（右寄せ）
3. **セクション区切り**: お気に入りユーザーのブロックと一般ユーザーのブロックの間に、ラベル付きのセパレーター（「─── みんなの状況 ───」のようなテキスト区切り線）を配置
4. **カード間余白の微調整**: お気に入りブロック内は `gap: 8px`、一般ブロック内は `gap: 12px` として微差をつける

### 2.8 Bootstrap変数オーバーライド（ _variables.scss）

```scss
// ブランドカラー（紫系グラデーションを避けたクリーンなブルー基調）
$primary:       #1A73E8;
$success:       #2E7D32;
$warning:       #F57F17;
$danger:        #D32F2F;
$info:          #1565C0;
$secondary:     #757575;
$light:         #F5F5F5;
$dark:          #212121;

// カスタム変数
$overdue-color: #E65100;
$favorite-active: #F9A825;
$favorite-inactive: #BDBDBD;
$border-color:  #E0E0E0;

// フォント
$font-family-sans-serif: 'Noto Sans JP', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;

// ボーダー半径（角丸を控えめに）
$border-radius:    6px;
$border-radius-lg: 8px;
$border-radius-sm: 4px;

// シャドウ（控えめ）
$box-shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.08);
$box-shadow:    0 2px 6px rgba(0, 0, 0, 0.1);
```

### 2.9 レスポンシブ設計

|ブレークポイント|対応|
|-|-|
|xs (< 576px)|シングルカラム。ナビゲーションはハンバーガーメニュー|
|sm (≥ 576px)|シングルカラム。カード幅100%|
|md (≥ 768px)|ダッシュボードのサマリー3カラム表示。コンテンツは2カラム|
|lg (≥ 992px)|フルレイアウト。サイドバーナビゲーション不要（トップナビ）|

 ---

## 3 . セキュリティ詳細設計

### 3.1 CORS設定（Actix-web）

```rust
Cors::default()
    .allowed _origin( &config.cors _origin)
    .allowed _methods(vec! ["GET", "POST", "PUT", "DELETE"])
    .allowed _headers(vec! [
        header::CONTENT _TYPE,
        header::AUTHORIZATION,
        HeaderName::from _static("x-requested-with"),
    ])
    .supports _credentials()  // Cookie送信許可
    .max _age(3600)
```

### 3.2 CSRF対策

* SameSite=Strict Cookieによりクロスサイトリクエストでのトークン送信をブロック
* 追加対策として、API呼び出し時に `X-Requested-With: XMLHttpRequest` ヘッダーをフロントエンドから送信し、バックエンドで検証

### 3.3 レート制限

* サインイン/サインアップ: 同一IPから1分間に5回まで
* その他のAPI: 同一セッションから1分間に60回まで
* Actix-webのミドルウェアで実装（インメモリカウンター）

### 3.4 入力サニタイゼーション

* HTMLタグのストリップ（ユーザー名・メモ等のテキストフィールド）
* SQLインジェクション: Supabaseクライアントのパラメータ化クエリで対策済み
* パストラバーサル: 該当するファイル操作なし

 ---

## 4 . ログ設計

### 4.1 ログレベルと出力内容

|レベル|出力内容|
|-|-|
|ERROR|500系エラー・DB接続失敗・外部API呼び出し失敗|
|WARN|認証失敗（不正トークン・パスワード不一致）・レート制限超過|
|INFO|サインアップ・サインイン・サインアウト・リソース作成/削除|
|DEBUG|APIリクエスト/レスポンス詳細（開発環境のみ）|

### 4.2 ログフォーマット（JSON構造化ログ）

```json
{
  "timestamp": "2025-01-01T00:00:00Z",
  "level": "INFO",
  "message": "User signed in",
  "user _id": "uuid",
  "ip": "xxx.xxx.xxx.xxx",
  "path": "/api/auth/signin",
  "method": "POST",
  "status": 200,
  "duration _ms": 45
}
```

注意: ログにパスワード・セッショントークン・メールアドレスの平文を含めないこと。

 ---

## 5 . デプロイ構成詳細

### 5.1 Dockerファイル構成

**バックエンド（マルチステージビルド）**:

```dockerfile
# ビルドステージ
FROM rust:1.78-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

# 実行ステージ
FROM debian:bookworm-slim
RUN apt-get update  & & apt-get install -y ca-certificates  & & rm -rf /var/lib/apt/lists/ *
COPY --from=builder /app/target/release/certmanager /usr/local/bin/
EXPOSE 8080
CMD  ["certmanager"]
```

**フロントエンド（ビルド後、バックエンドの静的ファイルとして配信）**:
フロントエンドのビルド成果物（dist/）をバックエンドのDockerイメージに含め、Actix-webの静的ファイルハンドラで配信する構成。これにより単一コンテナでの運用が可能。

### 5.2 環境変数一覧

|変数名|必須|説明|例|
|-|-|-|-|
|SUPABASE _URL|○|SupabaseプロジェクトのREST API URL|https://xxx.supabase.co/rest/v1|
|SUPABASE _KEY|○|Supabaseのanon key|eyJhbGci...|
|EMAIL _HMAC _SECRET|○|メールアドレスHMAC用シークレットキー（32バイト以上）|(ランダム文字列)|
|SERVER _PORT|×|サーバーポート（デフォルト: 8080）|8080|
|CORS _ORIGIN|○|許可するフロントエンドオリジン|https://certmanager.run.app|
|RUST _LOG|×|ログレベル（デフォルト: info）|info|



