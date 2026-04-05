use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::community_service;

#[derive(Deserialize)]
pub struct PaginationQuery {
    page: Option<i64>,
    per_page: Option<i64>,
}

pub async fn list_users(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let (users, total) = community_service::list_users(&db, user.0, page, per_page).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "users": users,
        "total": total,
        "page": page,
        "per_page": per_page,
    })))
}

pub async fn get_user(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let detail = community_service::get_user(&db, path.into_inner(), user.0).await?;
    Ok(HttpResponse::Ok().json(detail))
}
