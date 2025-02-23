//! # 認証サービス
//! 
//! 認証処理を定義したサービス
//! 
//! ## メソッド
//! 
//! `guest_login` - ゲストログイン
//! `signup`      - 新規登録
//! `login`       - ログイン

use async_trait::async_trait;
use argon2::Argon2;
use argon2::password_hash::{
    rand_core::OsRng,
    PasswordHash,
    PasswordHasher,
    PasswordVerifier,
    SaltString
};
use crate::domain::entities::auth::{LoginResponse, SignupResponse};
use crate::info_log;
use crate::{
    application::errors::auth_error::AuthError,
    application::jwt::jwt,
    application::types::di_type::AuthRepositoryArc,
    domain::entities::auth::{LoginRequest, SignupRequest},
    {app_log, error_log}
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register_user(&self, req: &SignupRequest) -> Result<(SignupResponse, String), AuthError>;
    async fn login_user(&self, req: &LoginRequest) -> Result<(LoginResponse, String), AuthError>;
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
    async fn register_user(&self, req: &SignupRequest) -> Result<(SignupResponse, String), AuthError> {
        // パスワードを暗号化
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = argon2.hash_password(&req.password.as_bytes(), &salt)?.to_string();
        
        // DB結果
        let insert_result = self.auth_repository.register_user(&req.name, &req.email, &hashed_password).await?;

        // JWT トークン生成
        let token = jwt::create_token(&insert_result.email, &insert_result.id)?;

        let response = SignupResponse {
            id: insert_result.id.to_string(),
            name: insert_result.name,
            email: insert_result.email,
            role: insert_result.role,
            photo: insert_result.photo,
            bio: insert_result.bio,
            is_verified: insert_result.is_verified,
            token: token.clone()
        };

        Ok((response, token))
    }

    async fn login_user(&self, req: &LoginRequest) -> Result<(LoginResponse, String), AuthError> {
        info_log!("[auth_service] - [login_user] login_user called");
        if let Some(select_result) = self.auth_repository.get_user_by_email(&req.email).await? {
            // パスワードのハッシュを検証
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&select_result.password)?;

            // 検証
            if let Err(err) = argon2.verify_password(req.password.as_bytes(), &parsed_hash) {
                error_log!("[auth_service] - [login] - [message: Authentication Failed] - Error: {:?}", err);
                return Err(AuthError::InvalidCredentials);
            }
    
            let token = jwt::create_token(&select_result.email, &select_result.id)?;

            let response = LoginResponse {
                id: select_result.id.to_string(),
                name: select_result.name,
                email: select_result.email,
                role: select_result.role,
                photo: select_result.photo,
                bio: select_result.bio,
                is_verified: select_result.is_verified,
                token: token.clone()
            };
    
            Ok((response, token))
        } else {
            error_log!("[auth_service] - [login] - [message: User Not Found]");
            Err(AuthError::UserNotFound)
        }
    }
}