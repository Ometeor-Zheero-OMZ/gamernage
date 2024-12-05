use async_trait::async_trait;use crate::domain::entities::{auth::LoginRequest, user::User};
use crate::application::errors::auth_error::AuthError;


#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    // async fn signup(&self, req: &SignupRequest, tx: &mut Transaction<'_>) -> Result<(), AuthError>;
    async fn signup(&self, name: &str, email: &str, hashed_password: &str) -> Result<(), AuthError>;
    // async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<(i32, String, String, String)>, AuthError>;
    
}