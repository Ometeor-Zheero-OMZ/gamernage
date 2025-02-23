use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::application::helpers::validator::validate_email;

#[derive(Debug)]
pub struct User {
   pub id: String,
   pub name: String,
   pub email: String,
   pub role: String,
   pub photo: Option<String>,
   pub bio: Option<String>,
   pub is_verified: bool,
}

// ユーザー取得　リクエスト
#[derive(Deserialize)]
pub struct UserRequest {
   pub user_id: String,
}

// ユーザー取得　DB結果
pub struct UserSelectResult {
   pub id: i32,
   pub name: String,
   pub email: String,
   pub role: String,
   pub photo: Option<String>,
   pub bio: Option<String>,
   pub is_verified: bool,
}

// ユーザー取得　レスポンス
#[derive(Serialize)]
pub struct UserResponse {
   pub id: String,
   pub name: String,
   pub email: String,
   pub role: String,
   pub photo: Option<String>,
   pub bio: Option<String>,
   pub is_verified: bool,
}

// パスワード忘れた　リクエスト
#[derive(Deserialize, Validate)]
pub struct ForgotPasswordRequest {
   #[validate(email(message = "Invalid email address"))]
   #[validate(length(max = 319, message = "Email address too long"))]
   #[validate(custom(function = "validate_email"))]
   pub email: String,
}