use std::collections::HashMap;
use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;

#[derive(Debug, serde::Serialize)]
pub struct CommunityCert {
    pub certification_name: String,
    pub acquired_date: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct CommunityGoal {
    pub certification_name: String,
    pub status: String,
    pub study_hours: f64,
    pub target_date: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CommunityUser {
    pub id: Uuid,
    pub username: String,
    pub certification_count: i64,
    pub goal_count: i64,
    pub achieved_count: i64,
    pub total_study_hours: f64,
    pub has_good_mark: bool,
    pub is_favorite: bool,
    pub certifications: Vec<CommunityCert>,
    pub goals: Vec<CommunityGoal>,
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
    // 1. Get all users except self
    let users_result = db
        .select(
            "TBL_USER",
            &format!("select=useid,usenm&useid=neq.{}&order=useca.desc", user_id),
        )
        .await?;
    let all_users: Vec<serde_json::Value> = serde_json::from_value(users_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    let total = all_users.len() as i64;

    // 2. Get holdings with master name
    let holdings_result = db
        .select("TBL_HOLDING", "select=holui,holdt,TBL_MASTER(masnm)")
        .await?;
    let holdings: Vec<serde_json::Value> = serde_json::from_value(holdings_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    let mut cert_counts: HashMap<String, i64> = HashMap::new();
    let mut certs_map: HashMap<String, Vec<CommunityCert>> = HashMap::new();
    for h in &holdings {
        if let Some(uid) = h["holui"].as_str() {
            *cert_counts.entry(uid.to_string()).or_insert(0) += 1;
            let cert_name = h["TBL_MASTER"]["masnm"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let acquired_date = h["holdt"].as_str().map(|s| s.to_string());
            certs_map
                .entry(uid.to_string())
                .or_insert_with(Vec::new)
                .push(CommunityCert {
                    certification_name: cert_name,
                    acquired_date,
                });
        }
    }

    // 3. Get goals with master name, status, study_hours
    let goals_result = db
        .select("TBL_GOAL", "select=goaui,goast,goash,goatd,TBL_MASTER(masnm)")
        .await?;
    let goals: Vec<serde_json::Value> = serde_json::from_value(goals_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    let mut goal_counts: HashMap<String, i64> = HashMap::new();
    let mut passed_counts: HashMap<String, i64> = HashMap::new();
    let mut study_hours_map: HashMap<String, f64> = HashMap::new();
    let mut goals_map: HashMap<String, Vec<CommunityGoal>> = HashMap::new();

    for g in &goals {
        if let Some(uid) = g["goaui"].as_str() {
            *goal_counts.entry(uid.to_string()).or_insert(0) += 1;
            let hours = g["goash"].as_f64().unwrap_or(0.0);
            *study_hours_map.entry(uid.to_string()).or_insert(0.0) += hours;

            if g["goast"].as_str() == Some("passed") {
                *passed_counts.entry(uid.to_string()).or_insert(0) += 1;
            }

            let cert_name = g["TBL_MASTER"]["masnm"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let status = g["goast"].as_str().unwrap_or("").to_string();
            let target_date = g["goatd"].as_str().unwrap_or("").to_string();

            goals_map
                .entry(uid.to_string())
                .or_insert_with(Vec::new)
                .push(CommunityGoal {
                    certification_name: cert_name,
                    status,
                    study_hours: hours,
                    target_date,
                });
        }
    }

    // 4. Get favorites for current user
    let favs_result = db
        .select(
            "TBL_FAVORITE",
            &format!("select=favti&favui=eq.{}", user_id),
        )
        .await?;
    let favs: Vec<serde_json::Value> = serde_json::from_value(favs_result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    let fav_set: Vec<String> = favs
        .iter()
        .filter_map(|f| f["favti"].as_str().map(|s| s.to_string()))
        .collect();

    // 5. Build community users with pagination
    let offset = ((page - 1) * per_page) as usize;
    let users: Vec<CommunityUser> = all_users
        .iter()
        .filter_map(|u| {
            let uid_str = u["useid"].as_str()?;
            let uid = Uuid::parse_str(uid_str).ok()?;
            let cert_count = *cert_counts.get(uid_str).unwrap_or(&0);
            let goal_count = *goal_counts.get(uid_str).unwrap_or(&0);
            let achieved_count = *passed_counts.get(uid_str).unwrap_or(&0);
            let total_study_hours = *study_hours_map.get(uid_str).unwrap_or(&0.0);
            let is_favorite = fav_set.contains(&uid_str.to_string());
            let user_certs = certs_map.remove(uid_str).unwrap_or_default();
            let user_goals = goals_map.remove(uid_str).unwrap_or_default();

            Some(CommunityUser {
                id: uid,
                username: u["usenm"].as_str()?.to_string(),
                certification_count: cert_count,
                goal_count,
                achieved_count,
                total_study_hours,
                has_good_mark: achieved_count > 0,
                is_favorite,
                certifications: user_certs,
                goals: user_goals,
            })
        })
        .skip(offset)
        .take(per_page as usize)
        .collect();

    Ok((users, total))
}

pub async fn get_user(
    db: &SupabaseClient,
    target_user_id: Uuid,
    _viewer_user_id: Uuid,
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

    // Get certifications with master name
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

    // Get goals with master name
    let goals = db
        .select(
            "TBL_GOAL",
            &format!(
                "select=goaid,goatd,goast,goamm,goash,goaca,TBL_MASTER(masid,masnm)&goaui=eq.{}&order=goaca.desc",
                target_user_id
            ),
        )
        .await?;
    let goals: Vec<serde_json::Value> = serde_json::from_value(goals)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // Check for passed goals
    let has_good_mark = goals.iter().any(|g| g["goast"].as_str() == Some("passed"));

    Ok(CommunityUserDetail {
        id: target_user_id,
        username: user["usenm"].as_str().unwrap_or_default().to_string(),
        has_good_mark,
        certifications: certs,
        goals,
    })
}
