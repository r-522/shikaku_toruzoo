use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;

#[derive(Debug, serde::Serialize)]
pub struct CommunityUser {
    pub id: Uuid,
    pub username: String,
    pub certification_count: i64,
    pub goal_count: i64,
    pub achieved_count: i64,
    pub has_good_mark: bool,
    pub is_favorite: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct CommunityUserDetail {
    pub id: Uuid,
    pub username: String,
    pub has_good_mark: bool,
    pub certifications: Vec<serde_json::Value>,
    pub goals: Vec<serde_json::Value>,
}

pub async fn list_users(
    db: &SupabaseClient,
    user_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<CommunityUser>, i64), AppError> {
    let offset = (page - 1) * per_page;

    let result = db
        .rpc(
            "get_community_users",
            &serde_json::json!({
                "p_user_id": user_id.to_string(),
                "p_limit": per_page,
                "p_offset": offset,
            }),
        )
        .await?;

    let rows: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    let users: Vec<CommunityUser> = rows
        .iter()
        .filter_map(|r| {
            Some(CommunityUser {
                id: Uuid::parse_str(r["user_id"].as_str()?).ok()?,
                username: r["username"].as_str()?.to_string(),
                certification_count: r["certification_count"].as_i64().unwrap_or(0),
                goal_count: r["goal_count"].as_i64().unwrap_or(0),
                achieved_count: r["achieved_count"].as_i64().unwrap_or(0),
                has_good_mark: r["has_good_mark"].as_bool().unwrap_or(false),
                is_favorite: r["is_favorite"].as_bool().unwrap_or(false),
            })
        })
        .collect();

    // Get total user count (excluding self)
    let count_result = db
        .select("TBL_USER", &format!("select=useid&useid=neq.{}", user_id))
        .await?;
    let all_users: Vec<serde_json::Value> = serde_json::from_value(count_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let total = all_users.len() as i64;

    Ok((users, total))
}

pub async fn get_user(
    db: &SupabaseClient,
    target_user_id: Uuid,
    viewer_user_id: Uuid,
) -> Result<CommunityUserDetail, AppError> {
    // Get user info
    let user_result = db
        .select(
            "TBL_USER",
            &format!("select=useid,usenm&useid=eq.{}", target_user_id),
        )
        .await?;
    let users: Vec<serde_json::Value> = serde_json::from_value(user_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;
    let user = users
        .first()
        .ok_or_else(|| AppError::NotFound("ユーザーが見つかりません".to_string()))?;

    // Get certifications
    let certs = db
        .select(
            "TBL_HOLDING",
            &format!(
                "select=holid,holdt,holca,TBL_MASTER(masid,masnm)&holui=eq.{}&order=holca.desc",
                target_user_id
            ),
        )
        .await?;
    let certs: Vec<serde_json::Value> = serde_json::from_value(certs)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // Get goals
    let goals = db
        .select(
            "TBL_GOAL",
            &format!(
                "select=goaid,goatd,goast,goamm,goaca,TBL_MASTER(masid,masnm)&goaui=eq.{}&order=goaca.desc",
                target_user_id
            ),
        )
        .await?;
    let goals: Vec<serde_json::Value> = serde_json::from_value(goals)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // Check for achieved goals
    let has_good_mark = goals.iter().any(|g| g["goast"].as_str() == Some("achieved"));

    Ok(CommunityUserDetail {
        id: target_user_id,
        username: user["usenm"].as_str().unwrap_or_default().to_string(),
        has_good_mark,
        certifications: certs,
        goals,
    })
}
