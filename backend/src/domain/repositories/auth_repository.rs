//! # 認証リポジトリ　インタフェース

use async_trait::async_trait;
use crate::{
    application::errors::auth_error::AuthError,
    domain::entities::auth::{LoginSelectResult, SignupInsertResult}
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn register_user(&self, name: &str, email: &str, password: &str) -> Result<SignupInsertResult, AuthError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<LoginSelectResult>, AuthError>;
}