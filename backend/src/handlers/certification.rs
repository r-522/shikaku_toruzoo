// ============================================================
// handlers/certification.rs — 所持資格ハンドラ
// ============================================================
// このファイルは「所持資格（TBL_HOLDING）」に関する HTTP リクエストを処理する。
//
// 【担当エンドポイント】
// GET    /api/certifications        — 所持資格一覧取得
// POST   /api/certifications        — 所持資格登録
// PUT    /api/certifications/{id}   — 所持資格更新
// DELETE /api/certifications/{id}   — 所持資格削除
//
// すべてのエンドポイントが `AuthenticatedUser` を引数に持つため、
// 未認証ユーザーには自動的に 401 Unauthorized が返される。

use actix_web::{web, HttpResponse};
// パスパラメータ（URL の `{id}` の部分）の型
use uuid::Uuid;
// バリデーション実行トレイト
use validator::Validate;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::certification::CertificationRequest;
use crate::services::cert_service;

// ============================================================
// list — GET /api/certifications
// ============================================================
/// ログイン中ユーザーの所持資格一覧を取得する
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー（user.0 = UUID）
///
/// # 戻り値
/// 200 OK + `{ "certifications": [...] }` JSON
pub async fn list(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // サービス層に処理を委譲してリストを取得
    let certs = cert_service::list(&db, user.0).await?;
    // `serde_json::json!({...})` でラッパーオブジェクトを作ってレスポンスにする
    Ok(HttpResponse::Ok().json(serde_json::json!({ "certifications": certs })))
}

// ============================================================
// create — POST /api/certifications
// ============================================================
/// 所持資格を新規登録する
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー
/// - `body`: リクエストボディ（certification_name, master_id?, acquired_date?）
///
/// # 戻り値
/// - 成功: 201 Created + 登録された資格情報 JSON
/// - バリデーション失敗: 422 Unprocessable Entity
pub async fn create(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    body: web::Json<CertificationRequest>,
) -> Result<HttpResponse, AppError> {
    // `#[validate]` アトリビュートで定義したルールを実行（例: 文字数チェック）
    body.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;

    let cert = cert_service::create(&db, user.0, &body).await?;
    // 201 Created（新規作成成功）でレスポンスを返す
    Ok(HttpResponse::Created().json(cert))
}

// ============================================================
// update — PUT /api/certifications/{id}
// ============================================================
/// 所持資格を更新する
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー
/// - `path`: URL パスパラメータ（`/api/certifications/{id}` の `id` 部分）
///   `web::Path<Uuid>` で自動的に UUID に変換される
/// - `body`: 更新内容
///
/// # 戻り値
/// - 成功: 200 OK + 更新後の資格情報 JSON
/// - 所有権なし: 404 Not Found
pub async fn update(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<CertificationRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;

    // `path.into_inner()` で `web::Path<Uuid>` から `Uuid` を取り出す
    let cert = cert_service::update(&db, user.0, path.into_inner(), &body).await?;
    Ok(HttpResponse::Ok().json(cert))
}

// ============================================================
// delete — DELETE /api/certifications/{id}
// ============================================================
/// 所持資格を削除する
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー
/// - `path`: 削除対象の UUID
///
/// # 戻り値
/// - 成功: 204 No Content（削除成功は body なし）
/// - 所有権なし: 404 Not Found
pub async fn delete(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    cert_service::delete(&db, user.0, path.into_inner()).await?;
    // 204 No Content（削除成功はレスポンスボディを持たない）
    // `.finish()` でボディなしのレスポンスを完成させる
    Ok(HttpResponse::NoContent().finish())
}
