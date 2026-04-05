use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct User {
    pub useid: Uuid,
    pub usenm: String,
    pub useml: String,
    pub usepw: String,
    pub useca: String,
    pub useua: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(length(min = 3, max = 30, message = "ユーザー名は3〜30文字で入力してください"))]
    pub username: String,
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
    pub email: String,
    #[validate(length(min = 8, message = "パスワードは8文字以上で入力してください"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SigninRequest {
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
    pub email: String,
    #[validate(length(min = 1, message = "パスワードを入力してください"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub created_at: String,
}
