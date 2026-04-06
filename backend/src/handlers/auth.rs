// ============================================================
// handlers/auth.rs — 認証ハンドラ
// ============================================================
// このファイルは認証に関する HTTP リクエストを受け取り、
// サービス層（auth_service.rs）に処理を委譲してレスポンスを返す。
//
// 【ハンドラの責務】
// - HTTP リクエストの型変換・バリデーション
// - サービス層の呼び出し
// - HTTP レスポンスの組み立て（ステータスコード・Cookie・JSON ボディ）
//
// 【Cookie セキュリティ設定】
// - HttpOnly: JavaScript から Cookie を読み取れない（XSS 対策）
// - Secure: HTTPS でのみ Cookie を送信（平文送信防止）
// - SameSite::Strict: 他サイトからのリクエストで Cookie を送らない（CSRF 対策）
//
// 【担当エンドポイント】
// POST /api/auth/signup — 新規登録
// POST /api/auth/signin — ログイン
// POST /api/auth/signout — ログアウト
// GET  /api/auth/me    — ログイン中ユーザー情報取得

// Actix-web の主要な型
use actix_web::{web, HttpRequest, HttpResponse};
// Cookie 関連の型（Actix-web の cookie モジュール）
use actix_web::cookie::{Cookie, SameSite};
// フィールドバリデーションのトレイト（`body.validate()` のために必要）
use validator::Validate;

use crate::config::Config;
use crate::db::SupabaseClient;
use crate::errors::AppError;
// 認証済みユーザーを表す型（引数に書くと自動的にセッション検証される）
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::{SigninRequest, SignupRequest};
use crate::services::auth_service;

// ============================================================
// signup — POST /api/auth/signup
// ============================================================
/// 新規ユーザー登録ハンドラ
///
/// バリデーション → サービス呼び出し → 201 Created を返す。
///
/// # 引数（Actix-web が自動的に解決・注入する）
/// - `db`: `app_data` に登録された DB クライアント
/// - `config`: アプリ設定（HMAC シークレット等）
/// - `body`: リクエストボディを SignupRequest として解析した JSON
///
/// # 戻り値
/// - 成功: 201 Created + メッセージ JSON
/// - 失敗: AppError（自動的に HTTP エラーレスポンスに変換）
pub async fn signup(
    db: web::Data<SupabaseClient>,
    config: web::Data<Config>,
    body: web::Json<SignupRequest>,
) -> Result<HttpResponse, AppError> {
    // `validate()` で `#[validate(...)]` アトリビュートに基づくバリデーションを実行
    // 失敗時は ValidationError に変換して返す
    body.validate().map_err(|e| {
        AppError::ValidationError(format!("{}", e))
    })?;

    // ---- パスワード複雑性チェック ----
    // validator クレートの `#[validate]` では複雑性（大文字・小文字・数字の混在）
    // をチェックできないため、ここで手動でチェックする
    let pwd = &body.password;
    // `chars().any(|c| ...)` は「いずれかの文字が条件を満たすか」を確認
    let has_upper = pwd.chars().any(|c| c.is_uppercase()); // 大文字を含むか
    let has_lower = pwd.chars().any(|c| c.is_lowercase()); // 小文字を含むか
    let has_digit = pwd.chars().any(|c| c.is_ascii_digit()); // 数字を含むか
    if !has_upper || !has_lower || !has_digit {
        return Err(AppError::ValidationError(
            "パスワードは英大文字・英小文字・数字をそれぞれ1文字以上含めてください".to_string(),
        ));
    }

    // サービス層に処理を委譲（メールハッシュ化・重複チェック・パスワードハッシュ化・INSERT）
    auth_service::signup(&db, &config, &body).await?;

    // 201 Created + 成功メッセージを返す
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "アカウントが作成されました"
    })))
}

// ============================================================
// signin — POST /api/auth/signin
// ============================================================
/// ログインハンドラ
///
/// 認証成功時にセッション Cookie を発行する。
///
/// # 引数
/// - `db`: DB クライアント
/// - `config`: HMAC シークレットを含む設定
/// - `body`: サインインリクエスト（email, password）
///
/// # 戻り値
/// - 成功: 200 OK + セッション Cookie + ユーザー情報 JSON
/// - 失敗: 401 Unauthorized
pub async fn signin(
    db: web::Data<SupabaseClient>,
    config: web::Data<Config>,
    body: web::Json<SigninRequest>,
) -> Result<HttpResponse, AppError> {
    // メールアドレス・パスワードの形式バリデーション
    body.validate().map_err(|e| {
        AppError::ValidationError(format!("{}", e))
    })?;

    // サービス層でメール・パスワード照合・セッション生成
    // 成功時は (セッショントークン, ユーザーID, ユーザー名) のタプルが返る
    let (token, user_id, username) = auth_service::signin(&db, &config, &body).await?;

    // ---- セッション Cookie の組み立て ----
    // `Cookie::build(名前, 値)` で Cookie ビルダーを作成
    let cookie = Cookie::build("session_token", token)
        .path("/")           // このドメインのすべてのパスで Cookie を送信
        .http_only(true)     // JavaScript からの読み取りを禁止（XSS 対策）
        .secure(true)        // HTTPS 接続でのみ Cookie を送信
        .same_site(SameSite::Strict) // 他サイトからのリクエストでは Cookie を送らない（CSRF 対策）
        // `max_age`: Cookie の有効期限（31536000 秒 = 1 年）
        .max_age(actix_web::cookie::time::Duration::seconds(31536000))
        .finish(); // Cookie を完成させる

    // Cookie をレスポンスヘッダーに付けて返す
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({
            "user": {
                "id": user_id,       // UUID（フロントエンドがユーザーIDを知るため）
                "username": username, // ユーザー名（フロントエンドが表示するため）
            }
        })))
}

// ============================================================
// signout — POST /api/auth/signout
// ============================================================
/// ログアウトハンドラ
///
/// DB からセッションを削除し、Cookie を無効化する。
///
/// # 引数
/// - `db`: DB クライアント
/// - `req`: HTTP リクエスト（Cookie を取り出すため）
/// - `_user`: 認証チェック用（引数に書くことで「未ログインなら 401」を強制）
///   `_` プレフィックスは「この変数は使わない」という意味
///
/// # 戻り値
/// - 成功: 200 OK + 空の Cookie（有効期限 0）+ メッセージ
pub async fn signout(
    db: web::Data<SupabaseClient>,
    req: HttpRequest,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Cookie からセッショントークンを取り出して DB のセッションを削除
    // `if let Some(cookie)` は Cookie が存在する場合のみ削除処理を行う
    if let Some(cookie) = req.cookie("session_token") {
        auth_service::signout(&db, cookie.value()).await?;
    }

    // ---- Cookie を無効化する ----
    // max_age を 0 にした同名 Cookie を返すことでブラウザに削除を指示する
    // `Duration::ZERO` = 0 秒（即座に期限切れ）
    let cookie = Cookie::build("session_token", "")
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::ZERO) // 期限切れ Cookie でブラウザに削除させる
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({
            "message": "サインアウトしました"
        })))
}

// ============================================================
// me — GET /api/auth/me
// ============================================================
/// ログイン中ユーザーの情報を返すハンドラ
///
/// フロントエンドがページロード時に「誰がログインしているか」を確認するために使う。
/// `AuthenticatedUser` 引数により、未認証なら自動的に 401 が返される。
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー（user.0 でユーザーIDを取り出せる）
///
/// # 戻り値
/// 200 OK + `UserResponse` JSON（id, username, created_at）
pub async fn me(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // `user.0` でタプル構造体の最初のフィールド（UUID）を取り出す
    let user_resp = auth_service::get_me(&db, user.0).await?;
    Ok(HttpResponse::Ok().json(user_resp))
}
