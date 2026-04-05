use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct Holding {
    pub holid: Uuid,
    pub holui: Uuid,
    pub holmi: Uuid,
    pub holdt: Option<String>,
    pub holca: String,
    pub holua: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CertificationRequest {
    #[validate(length(min = 1, max = 200, message = "資格名は1〜200文字で入力してください"))]
    pub certification_name: String,
    pub master_id: Option<Uuid>,
    pub acquired_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CertificationResponse {
    pub id: Uuid,
    pub certification_name: String,
    pub master_id: Uuid,
    pub acquired_date: Option<String>,
    pub created_at: String,
}
