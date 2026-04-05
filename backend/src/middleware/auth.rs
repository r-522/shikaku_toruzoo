use actix_web::{dev::Payload, FromRequest, HttpRequest, web};
use std::future::{Ready, ready};
use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub Uuid);

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = req
                .cookie("session_token")
                .map(|c| c.value().to_string())
                .ok_or_else(|| AppError::Unauthorized("認証が必要です".to_string()))?;

            let db = req
                .app_data::<web::Data<SupabaseClient>>()
                .ok_or_else(|| AppError::Internal("Database client not configured".to_string()))?;

            let query = format!(
                "select=sesui&sestk=eq.{}&sesea=gt.now()",
                urlencoding::encode(&token)
            );
            let result = db.select("TBL_SESSION", &query).await?;

            let sessions: Vec<serde_json::Value> = serde_json::from_value(result)
                .map_err(|e| AppError::Internal(format!("Session parse error: {}", e)))?;

            let session = sessions
                .first()
                .ok_or_else(|| AppError::Unauthorized("無効なセッションです".to_string()))?;

            let user_id = session["sesui"]
                .as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| AppError::Internal("Session user ID parse error".to_string()))?;

            Ok(AuthenticatedUser(user_id))
        })
    }
}
