//! # ユーザーリポジトリ　インタフェース

use async_trait::async_trait;
use crate::{
    application::{errors::user_error::UserError, jwt::jwt::Claims},
    domain::entities::user::UserResponse
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_id(&self, user: &Claims) ->Result<Option<i32>, UserError>;
    async fn find_user_by_id(&self, user_id: &str) -> Result<Option<UserResponse>, UserError>;
}