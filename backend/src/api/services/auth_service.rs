use async_trait::async_trait;

use crate::{constants::custom_type::AuthRepositoryArc, db::models::{auth::{LoginRequest, SignupRequest}, user::User}, errors::custom_error::AuthError};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError>;
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
}

pub struct AuthServiceImpl {
    auth_repository: AuthRepositoryArc
}

impl AuthServiceImpl {
    pub fn new(auth_repository: AuthRepositoryArc) -> Self {
        AuthServiceImpl { auth_repository }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError> {
        let user_opt = self.auth_repository.guest_login(req).await?;
        match user_opt {
            Some(user) => Ok(Some(user)),
            None => Ok(None)
        }
    }

    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError> {
        self.auth_repository.signup(req).await
    }

    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError> {
        self.auth_repository.login(req).await
    }
}