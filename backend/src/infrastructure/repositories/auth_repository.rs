//! # 認証リポジトリ
//! 
//! 認証処理を定義したリポジトリ
//! 
//! ## メソッド
//! 
//! `guest_login`             - ゲストログイン
//! `signup`                  - 新規登録
//! `get_user_by_email`       - ユーザー検索

use async_trait::async_trait;
use tokio_postgres::NoTls;
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::{
    application::errors::auth_error::AuthError, domain::{entities::auth::{LoginSelectResult, SignupInsertResult}, repositories::auth_repository::AuthRepository}, app_log, info_log
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
    async fn register_user(&self, name: &str, email: &str, password: &str) -> Result<SignupInsertResult, AuthError> {
        let conn = self.pool.get().await?;

        let row = conn.query_one(
            r#"
                INSERT INTO users (
                    name,
                    email,
                    password
                ) VALUES (
                    $1,
                    $2,
                    $3
                )
                RETURNING *;
            "#,
            &[&name, &email, &password]
        ).await?;

        Ok(SignupInsertResult {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            role: row.get("role"),
            photo: row.get("photo"),
            bio: row.get("bio"),
            is_verified: row.get("is_verified"),
        })
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<LoginSelectResult>, AuthError> {
        info_log!("[auth_repository] - [get_user_by_email] get_user_by_email called");

        let conn = self.pool.get().await?;

        let row_opt = conn.query_opt(
            r#"
                SELECT 
                    id,
                    name,
                    email,
                    role,
                    photo,
                    bio,
                    is_verified
                FROM
                    users
                WHERE
                    email = $1;
            "#,
            &[&email]
        ).await;

        match row_opt {
            Ok(Some(row)) => {
                Ok(Some(LoginSelectResult {
                    id: row.get("id"),
                    name: row.get("name"),
                    email: row.get("email"),
                    password: row.get("password"),
                    role: row.get("role"),
                    photo: row.get("photo"),
                    bio: row.get("bio"),
                    is_verified: row.get("is_verified"),
                }))
            },
            Ok(None) => Ok(None),
            Err(err) => Err(AuthError::DatabaseError(err.into())),
        }
    }
}