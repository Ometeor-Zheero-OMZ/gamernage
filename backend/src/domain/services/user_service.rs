use async_trait::async_trait;
use crate::application::errors::user_error::UserError;
use crate::application::jwt::jwt::Claims;
use crate::application::types::di_type::UserRepositoryArc;
use crate::{app_log, error_log};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_id(&self, user: &Claims) -> Result<i32, UserError>;
}

pub struct UserServiceImpl {
    user_repository: UserRepositoryArc,
}

impl UserServiceImpl {
    pub fn new(user_repository: UserRepositoryArc) -> Self {
        UserServiceImpl { user_repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user_id(&self, user: &Claims) -> Result<i32, UserError> {
        error_log!("[service] get_user_id called");
        let user_id_opt= self.user_repository.get_user_id(user).await?;

        error_log!("[service] user_id_opt = {:?}", user_id_opt);
        match user_id_opt {
            Some(user_id) => {
                error_log!("user_id = {}", user_id);
                Ok(user_id)
            },
            None => {
                error_log!("[user_service] - user not found for email: {}", user.sub);
                Err(UserError::UserNotFound)
            }
        }
    }
}