use uuid::Uuid;

use crate::config::Config;
use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::models::user::{SigninRequest, SignupRequest, UserResponse};
use crate::utils::hash::{hash_email, hash_password, verify_password};
use crate::utils::token::generate_session_token;

pub async fn signup(
    db: &SupabaseClient,
    config: &Config,
    req: &SignupRequest,
) -> Result<(), AppError> {
    let email_hash = hash_email(&req.email, &config.email_hmac_secret);

    // Check for existing email
    let existing = db
        .select("TBL_USER", &format!("select=useid&useml=eq.{}", email_hash))
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(existing)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    if !existing.is_empty() {
        return Err(AppError::Conflict(
            "このメールアドレスは既に登録されています".to_string(),
        ));
    }

    let password_hash = hash_password(&req.password)?;

    db.insert(
        "TBL_USER",
        &serde_json::json!({
            "usenm": req.username,
            "useml": email_hash,
            "usepw": password_hash,
        }),
    )
    .await?;

    Ok(())
}

pub async fn signin(
    db: &SupabaseClient,
    config: &Config,
    req: &SigninRequest,
) -> Result<(String, Uuid, String), AppError> {
    let email_hash = hash_email(&req.email, &config.email_hmac_secret);

    let result = db
        .select(
            "TBL_USER",
            &format!("select=useid,usenm,usepw&useml=eq.{}", email_hash),
        )
        .await?;
    let users: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    let user = users.first().ok_or_else(|| {
        AppError::Unauthorized("メールアドレスまたはパスワードが正しくありません".to_string())
    })?;

    let stored_hash = user["usepw"]
        .as_str()
        .ok_or_else(|| AppError::Internal("Password field missing".to_string()))?;

    if !verify_password(stored_hash, &req.password)? {
        return Err(AppError::Unauthorized(
            "メールアドレスまたはパスワードが正しくありません".to_string(),
        ));
    }

    let user_id = user["useid"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| AppError::Internal("User ID parse error".to_string()))?;

    let username = user["usenm"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let token = generate_session_token();

    // Session expires in 1 year
    let expires_at = chrono::Utc::now() + chrono::Duration::days(365);

    db.insert(
        "TBL_SESSION",
        &serde_json::json!({
            "sesui": user_id.to_string(),
            "sestk": token,
            "sesea": expires_at.to_rfc3339(),
        }),
    )
    .await?;

    Ok((token, user_id, username))
}

pub async fn signout(db: &SupabaseClient, token: &str) -> Result<(), AppError> {
    db.delete(
        "TBL_SESSION",
        &format!("sestk=eq.{}", urlencoding::encode(token)),
    )
    .await?;
    Ok(())
}

pub async fn get_me(db: &SupabaseClient, user_id: Uuid) -> Result<UserResponse, AppError> {
    let result = db
        .select(
            "TBL_USER",
            &format!("select=useid,usenm,useca&useid=eq.{}", user_id),
        )
        .await?;
    let users: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    let user = users
        .first()
        .ok_or_else(|| AppError::NotFound("ユーザーが見つかりません".to_string()))?;

    Ok(UserResponse {
        id: Uuid::parse_str(user["useid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))?,
        username: user["usenm"].as_str().unwrap_or_default().to_string(),
        created_at: user["useca"].as_str().unwrap_or_default().to_string(),
    })
}
