// ============================================================
// handlers/favorite.rs — お気に入りハンドラ
// ============================================================
// このファイルは「お気に入り（TBL_FAVORITE）」に関する HTTP リクエストを処理する。
//
// 【担当エンドポイント】
// GET    /api/favorites             — お気に入り一覧取得
// POST   /api/favorites/{userId}    — お気に入り追加
// DELETE /api/favorites/{userId}    — お気に入り解除
//
// URL パスパラメータ `{userId}` は「お気に入りにする/解除する相手」の UUID。

use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::favorite_service;

// ============================================================
// add — POST /api/favorites/{userId}
// ============================================================
/// 別のユーザーをお気に入りに追加する
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー（お気に入り登録を行う自分）
/// - `path`: URL パスパラメータ（お気に入りにしたい相手の UUID）
///
/// # 戻り値
/// - 成功: 201 Created + メッセージ JSON
/// - 自己お気に入り: 422 Validation Error
/// - 重複登録: 409 Conflict（DB の UNIQUE 制約）
pub async fn add(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // `path.into_inner()` でパスパラメータから UUID を取り出す
    favorite_service::add(&db, user.0, path.into_inner()).await?;
    // 201 Created で成功を伝える
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "お気に入りに追加しました"
    })))
}

// ============================================================
// remove — DELETE /api/favorites/{userId}
// ============================================================
/// お気に入り登録を解除する
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー（お気に入り解除を行う自分）
/// - `path`: URL パスパラメータ（解除したい相手の UUID）
///
/// # 戻り値
/// - 成功: 204 No Content（削除成功はボディなし）
pub async fn remove(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    favorite_service::remove(&db, user.0, path.into_inner()).await?;
    // 204 No Content（解除成功はレスポンスボディを持たない）
    Ok(HttpResponse::NoContent().finish())
}

// ============================================================
// list — GET /api/favorites
// ============================================================
/// ログイン中ユーザーのお気に入り一覧を取得する
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー
///
/// # 戻り値
/// 200 OK + `{ "favorites": [...] }` JSON
/// ※ 各要素に TBL_USER の JOIN 結果（useid, usenm）が含まれる
pub async fn list(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let favorites = favorite_service::list(&db, user.0).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "favorites": favorites })))
}
