use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::application::helpers::validator::{validate_email, validate_password};

/// 新規登録　リクエスト
#[derive(Deserialize, Debug, Validate)]
pub struct SignupRequest {
    pub name: String,
    #[validate(email(message = "Invalid email address"))]
    #[validate(length(max = 319, message = "Email address too long"))]
    #[validate(custom(function = "validate_email"))]
    pub email: String,
    #[validate(length(max = 127, message = "Password too long"))]
    #[validate(custom(function = "validate_password"))]
    pub password: String,
}

/// 新規登録　DB結果
pub struct SignupInsertResult {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: Option<String>,
    pub bio: Option<String>,
    pub is_verified: String,
}

/// 新規登録　レスポンス
#[derive(Serialize, Debug)]
pub struct SignupResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: Option<String>,
    pub bio: Option<String>,
    pub is_verified: String,
    pub token: String,
}

/// ログイン　リクエスト
#[derive(Deserialize, Debug, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    #[validate(length(max = 319, message = "Email address too long"))]
    #[validate(custom(function = "validate_email"))]
    pub email: String,
    #[validate(length(max = 127, message = "Password too long"))]
    #[validate(custom(function = "validate_password"))]
    pub password: String,
}

/// 新規登録　DB結果
pub struct LoginSelectResult {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub photo: Option<String>,
    pub bio: Option<String>,
    pub is_verified: String,
}

/// 新規登録　レスポンス
#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: Option<String>,
    pub bio: Option<String>,
    pub is_verified: String,
    pub token: String,
}