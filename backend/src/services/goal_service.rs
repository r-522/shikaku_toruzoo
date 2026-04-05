use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::models::goal::{GoalRequest, GoalResponse, GoalUpdateRequest};
use crate::services::master_service;

const VALID_STATUSES: &[&str] = &["studying", "scheduled", "achieved", "suspended"];

pub async fn list(
    db: &SupabaseClient,
    user_id: Uuid,
) -> Result<Vec<GoalResponse>, AppError> {
    let result = db
        .select(
            "TBL_GOAL",
            &format!(
                "select=goaid,goami,goatd,goast,goamm,goaca,TBL_MASTER(masid,masnm)&goaui=eq.{}&order=goaca.desc",
                user_id
            ),
        )
        .await?;

    let goals: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    Ok(goals
        .iter()
        .filter_map(|g| {
            let master = &g["TBL_MASTER"];
            Some(GoalResponse {
                id: Uuid::parse_str(g["goaid"].as_str()?).ok()?,
                certification_name: master["masnm"].as_str()?.to_string(),
                master_id: Uuid::parse_str(master["masid"].as_str()?).ok()?,
                target_date: g["goatd"].as_str()?.to_string(),
                status: g["goast"].as_str()?.to_string(),
                memo: g["goamm"].as_str().map(|s| s.to_string()),
                created_at: g["goaca"].as_str()?.to_string(),
            })
        })
        .collect())
}

pub async fn create(
    db: &SupabaseClient,
    user_id: Uuid,
    req: &GoalRequest,
) -> Result<GoalResponse, AppError> {
    let master_id = match req.master_id {
        Some(id) => id,
        None => master_service::find_or_create(db, &req.certification_name, "その他").await?,
    };

    let status = req.status.as_deref().unwrap_or("studying");
    if !VALID_STATUSES.contains(&status) {
        return Err(AppError::ValidationError(
            "無効なステータスです".to_string(),
        ));
    }

    let result = db
        .insert(
            "TBL_GOAL",
            &serde_json::json!({
                "goaui": user_id.to_string(),
                "goami": master_id.to_string(),
                "goatd": req.target_date,
                "goast": status,
                "goamm": req.memo,
            }),
        )
        .await?;

    let created: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let goal = created
        .first()
        .ok_or_else(|| AppError::Internal("Insert returned no data".to_string()))?;

    Ok(GoalResponse {
        id: Uuid::parse_str(goal["goaid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))?,
        certification_name: req.certification_name.clone(),
        master_id,
        target_date: req.target_date.clone(),
        status: status.to_string(),
        memo: req.memo.clone(),
        created_at: goal["goaca"].as_str().unwrap_or_default().to_string(),
    })
}

pub async fn update(
    db: &SupabaseClient,
    user_id: Uuid,
    goal_id: Uuid,
    req: &GoalUpdateRequest,
) -> Result<GoalResponse, AppError> {
    // Ownership check
    let existing = db
        .select(
            "TBL_GOAL",
            &format!(
                "select=goaid,goami,goatd,goast,goamm,goaca,TBL_MASTER(masid,masnm)&goaid=eq.{}&goaui=eq.{}",
                goal_id, user_id
            ),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let current = existing
        .first()
        .ok_or_else(|| AppError::NotFound("目標が見つかりません".to_string()))?;

    if let Some(ref status) = req.status {
        if !VALID_STATUSES.contains(&status.as_str()) {
            return Err(AppError::ValidationError(
                "無効なステータスです".to_string(),
            ));
        }
    }

    let mut update_body = serde_json::Map::new();
    if let Some(ref target_date) = req.target_date {
        update_body.insert("goatd".to_string(), serde_json::json!(target_date));
    }
    if let Some(ref status) = req.status {
        update_body.insert("goast".to_string(), serde_json::json!(status));
    }
    if let Some(ref memo) = req.memo {
        update_body.insert("goamm".to_string(), serde_json::json!(memo));
    }

    let result = db
        .update(
            "TBL_GOAL",
            &format!("goaid=eq.{}&goaui=eq.{}", goal_id, user_id),
            &serde_json::Value::Object(update_body),
        )
        .await?;

    let updated: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let goal = updated
        .first()
        .ok_or_else(|| AppError::Internal("Update returned no data".to_string()))?;

    let master = &current["TBL_MASTER"];
    let new_status = goal["goast"].as_str().unwrap_or("studying");

    Ok(GoalResponse {
        id: goal_id,
        certification_name: master["masnm"].as_str().unwrap_or_default().to_string(),
        master_id: Uuid::parse_str(master["masid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))?,
        target_date: goal["goatd"].as_str().unwrap_or_default().to_string(),
        status: new_status.to_string(),
        memo: goal["goamm"].as_str().map(|s| s.to_string()),
        created_at: goal["goaca"].as_str().unwrap_or_default().to_string(),
    })
}

pub async fn delete(
    db: &SupabaseClient,
    user_id: Uuid,
    goal_id: Uuid,
) -> Result<(), AppError> {
    let existing = db
        .select(
            "TBL_GOAL",
            &format!("select=goaid&goaid=eq.{}&goaui=eq.{}", goal_id, user_id),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    if existing.is_empty() {
        return Err(AppError::NotFound("目標が見つかりません".to_string()));
    }

    db.delete(
        "TBL_GOAL",
        &format!("goaid=eq.{}&goaui=eq.{}", goal_id, user_id),
    )
    .await?;

    Ok(())
}
