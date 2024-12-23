//! # 認証リポジトリ　インタフェース

use async_trait::async_trait;
use crate::{
    application::errors::auth_error::AuthError,
    domain::entities::auth::LoginRequest,
    domain::entities::user::User
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn signup(&self, name: &str, email: &str, hashed_password: &str) -> Result<(), AuthError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<(i32, String, String, String)>, AuthError>;
    
}