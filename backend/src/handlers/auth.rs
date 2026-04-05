use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::cookie::{Cookie, SameSite};
use validator::Validate;

use crate::config::Config;
use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user::{SigninRequest, SignupRequest};
use crate::services::auth_service;

pub async fn signup(
    db: web::Data<SupabaseClient>,
    config: web::Data<Config>,
    body: web::Json<SignupRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate().map_err(|e| {
        AppError::ValidationError(format!("{}", e))
    })?;

    // Validate password complexity
    let pwd = &body.password;
    let has_upper = pwd.chars().any(|c| c.is_uppercase());
    let has_lower = pwd.chars().any(|c| c.is_lowercase());
    let has_digit = pwd.chars().any(|c| c.is_ascii_digit());
    if !has_upper || !has_lower || !has_digit {
        return Err(AppError::ValidationError(
            "パスワードは英大文字・英小文字・数字をそれぞれ1文字以上含めてください".to_string(),
        ));
    }

    auth_service::signup(&db, &config, &body).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "アカウントが作成されました"
    })))
}

pub async fn signin(
    db: web::Data<SupabaseClient>,
    config: web::Data<Config>,
    body: web::Json<SigninRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate().map_err(|e| {
        AppError::ValidationError(format!("{}", e))
    })?;

    let (token, user_id, username) = auth_service::signin(&db, &config, &body).await?;

    let cookie = Cookie::build("session_token", token)
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::seconds(31536000))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({
            "user": {
                "id": user_id,
                "username": username,
            }
        })))
}

pub async fn signout(
    db: web::Data<SupabaseClient>,
    req: HttpRequest,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    if let Some(cookie) = req.cookie("session_token") {
        auth_service::signout(&db, cookie.value()).await?;
    }

    let cookie = Cookie::build("session_token", "")
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({
            "message": "サインアウトしました"
        })))
}

pub async fn me(
    db: web::Data<SupabaseClient>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user_resp = auth_service::get_me(&db, user.0).await?;
    Ok(HttpResponse::Ok().json(user_resp))
}
