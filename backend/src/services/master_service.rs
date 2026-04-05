use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::models::master::MasterSearchResult;

pub fn normalize_name(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

pub async fn search(db: &SupabaseClient, query: &str) -> Result<Vec<MasterSearchResult>, AppError> {
    if query.len() < 2 {
        return Ok(vec![]);
    }

    let encoded_query = urlencoding::encode(query);
    let result = db
        .select(
            "TBL_MASTER",
            &format!(
                "select=masid,masnm,masct&masnm=ilike.*{}*&limit=10",
                encoded_query
            ),
        )
        .await?;

    let masters: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    Ok(masters
        .iter()
        .filter_map(|m| {
            Some(MasterSearchResult {
                id: Uuid::parse_str(m["masid"].as_str()?).ok()?,
                name: m["masnm"].as_str()?.to_string(),
                category: m["masct"].as_str()?.to_string(),
            })
        })
        .collect())
}

pub async fn find_or_create(
    db: &SupabaseClient,
    name: &str,
    category: &str,
) -> Result<Uuid, AppError> {
    let normalized = normalize_name(name);

    // Check existing
    let result = db
        .select(
            "TBL_MASTER",
            &format!("select=masid&masnr=eq.{}", urlencoding::encode(&normalized)),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    if let Some(master) = existing.first() {
        return Uuid::parse_str(master["masid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)));
    }

    // Create new
    let result = db
        .insert(
            "TBL_MASTER",
            &serde_json::json!({
                "masnm": name.trim(),
                "masct": category,
                "masnr": normalized,
            }),
        )
        .await?;

    let created: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    let master = created
        .first()
        .ok_or_else(|| AppError::Internal("Master creation returned no data".to_string()))?;

    Uuid::parse_str(master["masid"].as_str().unwrap_or_default())
        .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))
}
