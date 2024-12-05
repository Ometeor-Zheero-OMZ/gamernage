use async_trait::async_trait;
use tokio_postgres::NoTls;
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::{
    application::{errors::user_error::UserError, jwt::jwt::Claims},
    domain::repositories::user_repository::UserRepository,
    {app_log, info_log}
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
    async fn get_user_id(&self, user: &Claims) -> Result<Option<i32>, UserError> {
        info_log!("[repository] get_user_id");
        let conn = self.pool.get().await?;
    
        let row = conn.query_opt(
            r#"
                SELECT
                    user_id
                FROM
                    user_profiles
                WHERE
                    email = $1
            "#,
            &[&user.sub]
        ).await?;

        info_log!("[repository] row = {:?}", row);

        Ok(row.and_then(|r| {
            let user_id= r.get("user_id");
            info_log!("[repository] user_id = {:?}", user_id);
            user_id
        }))
    }
}