use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::favorite_service;

pub async fn add(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    favorite_service::add(&db, user.0, path.into_inner()).await?;
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "お気に入りに追加しました"
    })))
}

pub async fn remove(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    favorite_service::remove(&db, user.0, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn list(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let favorites = favorite_service::list(&db, user.0).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "favorites": favorites })))
}
