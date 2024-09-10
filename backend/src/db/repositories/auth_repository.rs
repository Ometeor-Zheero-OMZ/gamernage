use async_trait::async_trait;
use bcrypt::verify;
use tokio_postgres::{Client, Error, NoTls};
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::{
    api::jwt::jwt,
    db::models::{
        auth::LoginRequest,
        user::User
    }
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn guest_login(
        &self,
        req: &LoginRequest,
        client: &Client
    ) -> Result<Option<User>, Error>;
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
        client: &Client
    ) -> Result<Option<User>, Error> {
        let rows = client.query(
            "SELECT id, name, password FROM users WHERE name = $1;",
            &[&req.name]
        ).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let id: String = rows.get(0).unwrap().get("id");
        let password: String = rows.get(0).unwrap().get("password");

        if verify(&req.password, &password).is_err() {
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
            Err(_) => Ok(None),
        }
    }
}