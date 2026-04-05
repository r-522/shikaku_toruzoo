use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct Goal {
    pub goaid: Uuid,
    pub goaui: Uuid,
    pub goami: Uuid,
    pub goatd: String,
    pub goast: String,
    pub goamm: Option<String>,
    pub goash: Option<f64>,
    pub goaca: String,
    pub goaua: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GoalRequest {
    #[validate(length(min = 1, max = 200, message = "資格名は1〜200文字で入力してください"))]
    pub certification_name: String,
    pub master_id: Option<Uuid>,
    pub target_date: String,
    pub status: Option<String>,
    #[validate(length(max = 1000, message = "メモは1000文字以内で入力してください"))]
    pub memo: Option<String>,
    pub study_hours: Option<f64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GoalUpdateRequest {
    pub target_date: Option<String>,
    pub status: Option<String>,
    #[validate(length(max = 1000, message = "メモは1000文字以内で入力してください"))]
    pub memo: Option<String>,
    pub study_hours: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct GoalResponse {
    pub id: Uuid,
    pub certification_name: String,
    pub master_id: Uuid,
    pub target_date: String,
    pub status: String,
    pub memo: Option<String>,
    pub study_hours: f64,
    pub created_at: String,
}
