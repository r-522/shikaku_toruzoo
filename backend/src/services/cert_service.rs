use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::models::certification::{CertificationRequest, CertificationResponse};
use crate::services::master_service;

pub async fn list(
    db: &SupabaseClient,
    user_id: Uuid,
) -> Result<Vec<CertificationResponse>, AppError> {
    let result = db
        .select(
            "TBL_HOLDING",
            &format!(
                "select=holid,holmi,holdt,holca,TBL_MASTER(masid,masnm)&holui=eq.{}&order=holca.desc",
                user_id
            ),
        )
        .await?;

    let holdings: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    Ok(holdings
        .iter()
        .filter_map(|h| {
            let master = &h["TBL_MASTER"];
            Some(CertificationResponse {
                id: Uuid::parse_str(h["holid"].as_str()?).ok()?,
                certification_name: master["masnm"].as_str()?.to_string(),
                master_id: Uuid::parse_str(master["masid"].as_str()?).ok()?,
                acquired_date: h["holdt"].as_str().map(|s| s.to_string()),
                created_at: h["holca"].as_str()?.to_string(),
            })
        })
        .collect())
}

pub async fn create(
    db: &SupabaseClient,
    user_id: Uuid,
    req: &CertificationRequest,
) -> Result<CertificationResponse, AppError> {
    let master_id = match req.master_id {
        Some(id) => id,
        None => {
            master_service::find_or_create(db, &req.certification_name, "その他").await?
        }
    };

    let result = db
        .insert(
            "TBL_HOLDING",
            &serde_json::json!({
                "holui": user_id.to_string(),
                "holmi": master_id.to_string(),
                "holdt": req.acquired_date,
            }),
        )
        .await?;

    let created: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let holding = created
        .first()
        .ok_or_else(|| AppError::Internal("Insert returned no data".to_string()))?;

    Ok(CertificationResponse {
        id: Uuid::parse_str(holding["holid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))?,
        certification_name: req.certification_name.clone(),
        master_id,
        acquired_date: req.acquired_date.clone(),
        created_at: holding["holca"].as_str().unwrap_or_default().to_string(),
    })
}

pub async fn update(
    db: &SupabaseClient,
    user_id: Uuid,
    holding_id: Uuid,
    req: &CertificationRequest,
) -> Result<CertificationResponse, AppError> {
    // Ownership check
    let existing = db
        .select(
            "TBL_HOLDING",
            &format!("select=holid&holid=eq.{}&holui=eq.{}", holding_id, user_id),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    if existing.is_empty() {
        return Err(AppError::NotFound("資格が見つかりません".to_string()));
    }

    let master_id = match req.master_id {
        Some(id) => id,
        None => {
            master_service::find_or_create(db, &req.certification_name, "その他").await?
        }
    };

    let result = db
        .update(
            "TBL_HOLDING",
            &format!("holid=eq.{}&holui=eq.{}", holding_id, user_id),
            &serde_json::json!({
                "holmi": master_id.to_string(),
                "holdt": req.acquired_date,
            }),
        )
        .await?;

    let updated: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let holding = updated
        .first()
        .ok_or_else(|| AppError::Internal("Update returned no data".to_string()))?;

    Ok(CertificationResponse {
        id: holding_id,
        certification_name: req.certification_name.clone(),
        master_id,
        acquired_date: req.acquired_date.clone(),
        created_at: holding["holca"].as_str().unwrap_or_default().to_string(),
    })
}

pub async fn delete(
    db: &SupabaseClient,
    user_id: Uuid,
    holding_id: Uuid,
) -> Result<(), AppError> {
    // Ownership check
    let existing = db
        .select(
            "TBL_HOLDING",
            &format!("select=holid&holid=eq.{}&holui=eq.{}", holding_id, user_id),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    if existing.is_empty() {
        return Err(AppError::NotFound("資格が見つかりません".to_string()));
    }

    db.delete(
        "TBL_HOLDING",
        &format!("holid=eq.{}&holui=eq.{}", holding_id, user_id),
    )
    .await?;

    Ok(())
}
