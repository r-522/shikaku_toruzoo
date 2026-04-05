use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::services::master_service;

#[derive(Deserialize)]
pub struct SearchQuery {
    q: Option<String>,
}

pub async fn search(
    db: web::Data<SupabaseClient>,
    _user: AuthenticatedUser,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AppError> {
    let q = query.q.as_deref().unwrap_or("");
    let results = master_service::search(&db, q).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "certifications": results })))
}
