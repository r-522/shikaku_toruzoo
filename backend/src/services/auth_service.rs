// ============================================================
// services/auth_service.rs — 認証ビジネスロジック
// ============================================================
// このファイルはユーザー認証に関する処理を集約する「サービス層」。
//
// 【役割の分担】
// - ハンドラ（handlers/auth.rs）: HTTP リクエストの受け取りとレスポンスの返却
// - サービス（このファイル）: 実際のビジネスロジック（DB 操作・検証・計算）
// - DB クライアント（db.rs）: Supabase への HTTP リクエスト送信
//
// この分離により「ハンドラは薄く、サービスに処理を集める」設計になっている。
//
// 【担当する処理】
// - signup: 新規ユーザー登録（メールハッシュ・パスワードハッシュ生成）
// - signin: ログイン（パスワード照合・セッション発行）
// - signout: ログアウト（セッション削除）
// - get_me: ログイン中ユーザー情報取得

use uuid::Uuid;

use crate::config::Config;
use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::models::user::{SigninRequest, SignupRequest, UserResponse};
// パスワードハッシュ化・照合・メールハッシュ化
use crate::utils::hash::{hash_email, hash_password, verify_password};
// セッショントークン生成
use crate::utils::token::generate_session_token;

// ============================================================
// signup — 新規ユーザー登録
// ============================================================
/// 新規ユーザーを登録する
///
/// 処理の流れ:
/// 1. メールアドレスを HMAC-SHA256 でハッシュ化
/// 2. 同じハッシュ値が DB に存在しないか確認（重複チェック）
/// 3. パスワードを Argon2id でハッシュ化
/// 4. TBL_USER に INSERT
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `config`: HMAC シークレットを含む設定
/// - `req`: サインアップリクエスト（username, email, password）
///
/// # 戻り値
/// 成功時は `Ok(())`、失敗時は `AppError`
///
/// # セキュリティ
/// - メールアドレスは平文で保存しない（HMAC ハッシュのみ保存）
/// - パスワードは Argon2id でハッシュ化（ソルト自動生成）
pub async fn signup(
    db: &SupabaseClient,
    config: &Config,
    req: &SignupRequest,
) -> Result<(), AppError> {
    // メールアドレスを HMAC-SHA256 でハッシュ化する
    // 同じメールアドレスからは常に同じハッシュ値が生成されるため
    // 「既に登録されているか」の検索に使える
    let email_hash = hash_email(&req.email, &config.email_hmac_secret);

    // ---- メールアドレス重複チェック ----
    // `useml=eq.{email_hash}` は「useml フィールドがハッシュ値と等しい」という条件
    let existing = db
        .select("TBL_USER", &format!("select=useid&useml=eq.{}", email_hash))
        .await?;
    // JSON 配列を Vec<Value> に変換（`?` で失敗時はエラーを伝播）
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // 1件でもヒットした場合は重複エラーを返す
    if !existing.is_empty() {
        return Err(AppError::Conflict(
            "このメールアドレスは既に登録されています".to_string(),
        ));
    }

    // パスワードを Argon2id でハッシュ化する
    // 毎回異なるソルトが自動生成されるため、同じパスワードでも異なるハッシュになる
    let password_hash = hash_password(&req.password)?;

    // TBL_USER に新規レコードを挿入する
    // メールは平文ではなくハッシュ値（useml）を保存する
    db.insert(
        "TBL_USER",
        &serde_json::json!({
            "usenm": req.username,
            "useml": email_hash,
            "usepw": password_hash,
        }),
    )
    .await?;

    // 登録成功。セッションはまだ作成しない（signin で別途行う）
    Ok(())
}

// ============================================================
// signin — ログイン処理
// ============================================================
/// ユーザーをログインさせ、セッショントークンを発行する
///
/// 処理の流れ:
/// 1. メールアドレスをハッシュ化して DB を検索
/// 2. DB から取得したパスワードハッシュと入力パスワードを照合
/// 3. セッショントークンを生成して TBL_SESSION に保存
/// 4. トークン・ユーザーID・ユーザー名を返す
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `config`: HMAC シークレットを含む設定
/// - `req`: サインインリクエスト（email, password）
///
/// # 戻り値
/// `(セッショントークン, ユーザーID, ユーザー名)` のタプル
///
/// # セキュリティ
/// - メール・パスワード両方が正しくないと同じエラーを返す（どちらが間違っているか教えない）
/// - セッションは 1 年間有効
pub async fn signin(
    db: &SupabaseClient,
    config: &Config,
    req: &SigninRequest,
) -> Result<(String, Uuid, String), AppError> {
    // メールアドレスをハッシュ化して DB 検索に使う
    let email_hash = hash_email(&req.email, &config.email_hmac_secret);

    // TBL_USER からメールハッシュに一致するユーザーを取得
    // パスワードハッシュ（usepw）も一緒に取得する（照合のため）
    let result = db
        .select(
            "TBL_USER",
            &format!("select=useid,usenm,usepw&useml=eq.{}", email_hash),
        )
        .await?;
    let users: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // ユーザーが見つからない場合（注意: エラーメッセージはパスワードと同じにする）
    // → 「メールが間違っている」と教えるとメールアドレス列挙攻撃に使われる危険がある
    let user = users.first().ok_or_else(|| {
        AppError::Unauthorized("メールアドレスまたはパスワードが正しくありません".to_string())
    })?;

    // DB に保存されたパスワードハッシュを取り出す
    let stored_hash = user["usepw"]
        .as_str()
        .ok_or_else(|| AppError::Internal("Password field missing".to_string()))?;

    // Argon2id でパスワードを照合する
    // `verify_password(ハッシュ, 平文パスワード)` → true/false
    if !verify_password(stored_hash, &req.password)? {
        // パスワードが一致しない場合もメールと同じエラーメッセージ
        return Err(AppError::Unauthorized(
            "メールアドレスまたはパスワードが正しくありません".to_string(),
        ));
    }

    // ユーザーID を JSON 文字列から Uuid 型にパース
    let user_id = user["useid"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| AppError::Internal("User ID parse error".to_string()))?;

    // ユーザー名を取得（なければ空文字列）
    let username = user["usenm"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    // 暗号学的乱数で 256bit のセッショントークンを生成（Base64 エンコード済み）
    let token = generate_session_token();

    // セッションの有効期限を設定（現在時刻から 365 日後）
    // `chrono::Utc::now()` は UTC の現在時刻
    // `+ chrono::Duration::days(365)` で 1 年後の日時を計算
    // `.to_rfc3339()` は ISO 8601 形式の文字列に変換（例: "2025-01-01T00:00:00+00:00"）
    let expires_at = chrono::Utc::now() + chrono::Duration::days(365);

    // TBL_SESSION にセッションレコードを作成
    db.insert(
        "TBL_SESSION",
        &serde_json::json!({
            "sesui": user_id.to_string(),  // ユーザーID
            "sestk": token,                 // セッショントークン（平文）
            "sesea": expires_at.to_rfc3339(), // 有効期限
        }),
    )
    .await?;

    // ハンドラ側で Cookie を設定するためにトークンを返す
    Ok((token, user_id, username))
}

// ============================================================
// signout — ログアウト処理
// ============================================================
/// セッションを削除してログアウトする
///
/// Cookie のトークンに一致するセッションレコードを TBL_SESSION から削除する。
/// セッション削除後は同じトークンでの認証が不可能になる。
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `token`: 削除するセッショントークン（Cookie から取得したもの）
///
/// # 戻り値
/// 成功時は `Ok(())`
///
/// # 注意
/// トークンが DB に存在しない場合でもエラーにしない
/// （冪等性を保つ: 同じ操作を何度やっても結果が変わらない）
pub async fn signout(db: &SupabaseClient, token: &str) -> Result<(), AppError> {
    // `urlencoding::encode(token)` で URL に使えない文字（+ 等）をエスケープ
    // トークンは Base64 文字列なので + と / と = を含む可能性がある
    db.delete(
        "TBL_SESSION",
        &format!("sestk=eq.{}", urlencoding::encode(token)),
    )
    .await?;
    Ok(())
}

// ============================================================
// get_me — ログイン中ユーザー情報取得
// ============================================================
/// ログイン中のユーザー情報を取得する
///
/// セッション検証済みのユーザーID を使って TBL_USER を検索し、
/// ユーザー情報を返す。
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 認証済みユーザーの UUID
///
/// # 戻り値
/// ユーザー情報（`UserResponse`）
pub async fn get_me(db: &SupabaseClient, user_id: Uuid) -> Result<UserResponse, AppError> {
    // ユーザーID で TBL_USER を検索
    // パスワードハッシュ（usepw）やメールハッシュ（useml）は返さない（必要最小限のみ）
    let result = db
        .select(
            "TBL_USER",
            &format!("select=useid,usenm,useca&useid=eq.{}", user_id),
        )
        .await?;
    let users: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // セッション検証済みなので必ず存在するはずだが、念のため NotFound エラーを返す
    let user = users
        .first()
        .ok_or_else(|| AppError::NotFound("ユーザーが見つかりません".to_string()))?;

    // `UserResponse` 構造体を組み立てて返す
    Ok(UserResponse {
        id: Uuid::parse_str(user["useid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))?,
        username: user["usenm"].as_str().unwrap_or_default().to_string(),
        created_at: user["useca"].as_str().unwrap_or_default().to_string(),
    })
}
