use async_trait::async_trait;
use bcrypt::verify;
use tokio_postgres::NoTls;
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::{
    application::jwt::jwt,
    application::errors::auth_error::AuthError,
    domain::repositories::auth_repository::AuthRepository,
    domain::entities::{auth::LoginRequest, user::User},
    {app_log, error_log}
};

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
    // 本番では使用しないためロジックはアーキテクトを無視する
    async fn guest_login(
        &self,
        req: &LoginRequest,
    ) -> Result<Option<User>, AuthError> {
        let conn = self.pool.get().await?;
        
        let rows = conn.query(
            r#"
                SELECT 
                    users.id,
                    user_profiles.name,
                    user_profiles.email,
                    user_auth.password
                FROM
                    users
                INNER JOIN
                    user_auth
                ON
                    user_auth.user_id = users.id
                INNER JOIN
                    user_profiles
                ON
                    user_profiles.user_id = user_auth.user_id
                WHERE
                    email = $1;
            "#,
            &[&req.email]
        ).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let id: i32 = rows.get(0).unwrap().get("id");
        let password: String = rows.get(0).unwrap().get("password");

        if verify(&req.password, &password).is_err() {
            error_log!("[auth_repository] - [guest_login] - [Authentication Failed]");
            return Ok(None);
        }

        match jwt::create_token(&req.email, &id) {
            Ok(token) => {
                let user_data = User {
                    id,
                    name: req.name.clone(),
                    email: req.email.clone(),
                    token,
                };
                Ok(Some(user_data))
            }
            Err(error) => {
                error_log!("[auth_repository] - [guest_login] - [message: error = {}]", error);
                Err(AuthError::TokenCreationError(error))
            }
        }
    }

    async fn signup(
        &self,
        name: &str,
        email: &str,
        hashed_password: &str,
    ) -> Result<(), AuthError> {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(AuthError::from)?;
        let tx = conn.transaction().await.map_err(AuthError::from)?;

        let result_row = tx.query_one(
            r#"
                INSERT INTO users (
                    created_at,
                    updated_at
                ) VALUES (
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                )
                RETURNING id;
            "#,
            &[]
        ).await?;

        let user_id: i32 = result_row.get("id");

        tx.execute(
            r#"
                INSERT INTO user_profiles (
                    user_id,
                    name,
                    email,
                    created_at,
                    updated_at
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                );
            "#,
            &[&user_id, &name, &email]
        ).await?;

        tx.execute(
            r#"
                INSERT INTO user_auth (
                    user_id,
                    password,
                    created_at,
                    updated_at
                ) VALUES (
                    $1,
                    $2,
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                );
            "#,
            &[&user_id, &hashed_password]
        ).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<(i32, String, String, String)>, AuthError> {
        let conn = self.pool.get().await?;
        let rows = conn.query(
            r#"
                SELECT 
                    users.id,
                    user_profiles.name,
                    user_profiles.email,
                    user_auth.password
                FROM
                    users
                INNER JOIN
                    user_auth
                ON
                    user_auth.user_id = users.id
                INNER JOIN
                    user_profiles
                ON
                    user_profiles.user_id = user_auth.user_id
                WHERE
                    user_profiles.email = $1;
            "#,
            &[&email]
        ).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let id: i32 = rows.get(0).unwrap().get("id");
        let name: String = rows.get(0).unwrap().get("name");
        let email: String = rows.get(0).unwrap().get("email");
        let password: String = rows.get(0).unwrap().get("password");

        Ok(Some((id, name, email, password)))
    }
}