//! # ユーザーリポジトリ
//! 
//! ユーザー処理を定義したリポジトリ
//! 
//! ## メソッド
//! 
//! `get_user_id` - ユーザーID取得

use async_trait::async_trait;
use tokio_postgres::NoTls;
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::{
    application::{errors::user_error::UserError, jwt::jwt::Claims},
    domain::{entities::user::UserResponse, repositories::user_repository::UserRepository},
};

pub struct UserRepositoryImpl {
    pool: Pool<PostgresConnectionManager<NoTls>>
}

impl UserRepositoryImpl {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        UserRepositoryImpl { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    /// ユーザーID取得
    /// 
    /// メールアドレスをキーにユーザーIDを取得します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ。`sub` フィールドにメールアドレスが格納されています。
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(Some(i32)`   - ユーザーIDを取得した場合、ユーザーIDを返します。
    /// - `Ok(None)`       - メールアドレスに該当するユーザーが見つからなかった場合、`None` を返します。
    /// - `Err(UserError)` - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn get_user_id(&self, user: &Claims) -> Result<Option<i32>, UserError> {
        let conn = self.pool.get().await?;
    
        let row = conn.query_opt(
            r#"
                SELECT
                    user_id
                FROM
                    users
                WHERE
                    email = $1
            "#,
            &[&user.sub]
        ).await?;

        Ok(row.and_then(|r| {
            let user_id= r.get("user_id");
            user_id
        }))
    }

    async fn find_user_by_id(&self, user_id: &str) -> Result<Option<UserResponse>, UserError> {
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
                    id = $1;
            "#,
            &[&user_id]
        ).await;

        match row_opt {
            Ok(Some(row)) => {
                Ok(Some(UserResponse {
                    id: row.get("id"),
                    name: row.get("name"),
                    email: row.get("email"),
                    role: row.get("role"),
                    photo: row.get("photo"),
                    bio: row.get("bio"),
                    is_verified: row.get("is_verified"),
                }))
            },
            Ok(None) => Ok(None),
            Err(err) => Err(UserError::DatabaseError(err.into())),
        } 
    }
}