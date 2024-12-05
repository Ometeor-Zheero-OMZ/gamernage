use async_trait::async_trait;
use crate::application::{errors::user_error::UserError, jwt::jwt::Claims};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_id(&self, user: &Claims) ->Result<Option<i32>, UserError>;
}