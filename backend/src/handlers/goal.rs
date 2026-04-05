use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::goal::{GoalRequest, GoalUpdateRequest};
use crate::services::goal_service;

pub async fn list(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let goals = goal_service::list(&db, user.0).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "goals": goals })))
}

pub async fn create(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    body: web::Json<GoalRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;

    let goal = goal_service::create(&db, user.0, &body).await?;
    Ok(HttpResponse::Created().json(goal))
}

pub async fn update(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<GoalUpdateRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;

    let goal = goal_service::update(&db, user.0, path.into_inner(), &body).await?;
    Ok(HttpResponse::Ok().json(goal))
}

pub async fn delete(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    goal_service::delete(&db, user.0, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
