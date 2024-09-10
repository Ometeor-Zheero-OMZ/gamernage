use async_trait::async_trait;
use bcrypt::verify;
use tokio_postgres::NoTls;
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
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
    api::jwt::jwt,
    db::models::{
        auth::{
            LoginRequest,
            SignupRequest
        },
        user::User
    },
    errors::custom_error::AuthError,
    libraries::logger
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError>;
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
}

pub struct AuthRepositoryImpl {
    pool: Pool<PostgresConnectionManager<NoTls>>
}

impl AuthRepositoryImpl {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        AuthRepositoryImpl { pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn guest_login(
        &self,
        req: &LoginRequest,
    ) -> Result<Option<User>, AuthError> {
        let conn = self.pool.get().await?;

        let rows = conn.query(
            "SELECT id, name, password FROM users WHERE name = $1;",
            &[&req.name]
        ).await?;

        // ユーザーが存在しない場合
        if rows.is_empty() {
            return Ok(None);
        }

        let id: String = rows.get(0).unwrap().get("id");
        let password: String = rows.get(0).unwrap().get("password");

        // パスワードが一致しない場合
        if verify(&req.password, &password).is_err() {
            logger::log(logger::Header::ERROR, "Invalid password attempt");
            return Ok(None);
        }

        match jwt::create_token(&req.name, &id) {
            Ok(token) => {
                let user_data = User {
                    id,
                    name: req.name.clone(),
                    token,
                };
                Ok(Some(user_data))
            }
            Err(err) => {
                logger::log(logger::Header::ERROR, &err.to_string());
                Err(AuthError::TokenCreationError(err))
            }
        }
    }

    async fn signup(
        &self,
        req: &SignupRequest
    ) -> Result<(), AuthError> {
        let conn = self.pool.get().await?;

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = argon2.hash_password(&req.password.as_bytes(), &salt)?.to_string();
        
        conn.execute(
            "INSERT INTO users (name, password, email) VALUES ($1, $2, $3)",
            &[&req.name, &hashed_password, &req.email]
        ).await?;

        Ok(())
    }

    async fn login(
        &self,
        req: &LoginRequest
    ) -> Result<Option<User>, AuthError> {
        let conn = self.pool.get().await?;

        let rows = conn.query(
            "SELECT id, name, password FROM users WHERE name = $1;",
            &[&req.name]
        ).await?;

        // ユーザーが存在しない場合
        if rows.is_empty() {
            return Ok(None);
        }

        let id: String = rows.get(0).unwrap().get("id");
        let password: String = rows.get(0).unwrap().get("password");

        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&password)?;

        argon2.verify_password(req.password.as_bytes(), &parsed_hash)?;

        let token = jwt::create_token(&req.name, &id)?;

        let user_data = User {
            id,
            name: req.name.clone(),
            token,
        };

        Ok(Some(user_data))
    }
}