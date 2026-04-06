// ============================================================
// handlers/community.rs — コミュニティハンドラ
// ============================================================
// このファイルは「コミュニティ」機能（他ユーザーの公開情報閲覧）の
// HTTP リクエストを処理する。
//
// 【担当エンドポイント】
// GET /api/community/users           — ユーザー一覧（ページネーション付き）
// GET /api/community/users/{id}      — ユーザー詳細
//
// 【ページネーションパラメータ】
// ?page=1&per_page=20 の形式で指定する。
// - page: ページ番号（1 始まり、未指定なら 1）
// - per_page: 1 ページあたりの件数（未指定なら 20、最大 100 件に制限）

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::community_service;

// ============================================================
// PaginationQuery — ページネーションクエリパラメータ
// ============================================================
/// GET /api/community/users?page=1&per_page=20 のクエリパラメータ
///
/// `Option<i64>` にすることで、未指定の場合はデフォルト値を使う。
#[derive(Deserialize)]
pub struct PaginationQuery {
    /// ページ番号（1 始まり）
    page: Option<i64>,
    /// 1 ページあたりの件数
    per_page: Option<i64>,
}

// ============================================================
// list_users — GET /api/community/users
// ============================================================
/// コミュニティのユーザー一覧を取得する
///
/// 自分以外のすべてのユーザーと、その資格・目標の概要を返す。
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー（自分自身を除外するため）
/// - `query`: ページネーションパラメータ
///
/// # 戻り値
/// 200 OK + `{ "users": [...], "total": n, "page": n, "per_page": n }` JSON
pub async fn list_users(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    // page: 未指定なら 1、1 より小さい値なら 1 に切り上げ（`.max(1)`）
    let page = query.page.unwrap_or(1).max(1);

    // per_page: 未指定なら 20、1〜100 の範囲に制限（`.clamp(最小, 最大)`）
    // clamp は「指定範囲内に値を収める」関数（min/max の組み合わせ）
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    // サービス層で全データを取得・集計・ページネーション適用
    let (users, total) = community_service::list_users(&db, user.0, page, per_page).await?;

    // ページネーション情報もレスポンスに含める（フロントエンドが総ページ数を計算するため）
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "users": users,
        "total": total,       // 全件数（ページネーション前）
        "page": page,         // 現在のページ番号
        "per_page": per_page, // 1 ページあたりの件数
    })))
}

// ============================================================
// get_user — GET /api/community/users/{id}
// ============================================================
/// 特定ユーザーの詳細情報を取得する
///
/// # 引数
/// - `db`: DB クライアント
/// - `user`: 認証済みユーザー（閲覧者。将来の機能拡張で使用）
/// - `path`: URL パスパラメータ（対象ユーザーの UUID）
///
/// # 戻り値
/// - 成功: 200 OK + ユーザー詳細 JSON（資格・目標を含む）
/// - 存在しない: 404 Not Found
pub async fn get_user(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    // `path.into_inner()` で URL の `{id}` 部分の UUID を取り出す
    // `user.0` は閲覧者のID（サービス層で将来の閲覧制限等に使用する可能性がある）
    let detail = community_service::get_user(&db, path.into_inner(), user.0).await?;
    Ok(HttpResponse::Ok().json(detail))
}
