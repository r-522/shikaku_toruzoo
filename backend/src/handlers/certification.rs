use actix_web::{web, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::certification::CertificationRequest;
use crate::services::cert_service;

pub async fn list(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let certs = cert_service::list(&db, user.0).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "certifications": certs })))
}

pub async fn create(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    body: web::Json<CertificationRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;

    let cert = cert_service::create(&db, user.0, &body).await?;
    Ok(HttpResponse::Created().json(cert))
}

pub async fn update(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<CertificationRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(format!("{}", e)))?;

    let cert = cert_service::update(&db, user.0, path.into_inner(), &body).await?;
    Ok(HttpResponse::Ok().json(cert))
}

pub async fn delete(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    cert_service::delete(&db, user.0, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
