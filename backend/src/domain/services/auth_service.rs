use async_trait::async_trait;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString
    },
    Argon2
};
use crate::{
    application::{errors::auth_error::AuthError, jwt::jwt, types::di_type::AuthRepositoryArc},
    domain::entities::{auth::{LoginRequest, SignupRequest}, user::User},
    {app_log, error_log}
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError>;
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
}

pub struct AuthServiceImpl {
    auth_repository: AuthRepositoryArc,
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
        // パスワードを暗号化
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = argon2.hash_password(&req.password.as_bytes(), &salt)?.to_string();
        
        self.auth_repository.signup(&req.name, &req.email, &hashed_password).await?;

        Ok(())
    }

    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError> {
        if let Some((id, name, email, hashed_password)) = self.auth_repository.get_user_by_email(&req.email).await? {
            // ハッシュ化されたパスワードをパース
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&hashed_password)?;

            // 検証
            if let Err(err) = argon2.verify_password(req.password.as_bytes(), &parsed_hash) {
                error_log!("[auth_service] - [login] - [message: Authentication Failed] - Error: {:?}", err);
                return Ok(None);
            }
    
            let token = jwt::create_token(&req.email, &id)?;
            let user_data = User {
                id,
                name,
                email,
                token,
            };
    
            return Ok(Some(user_data));
        }

        Ok(None)
    }
}