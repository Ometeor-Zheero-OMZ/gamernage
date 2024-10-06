use crate::db::models::community::Community;
use crate::errors::community_error::CommunityError;
use async_trait::async_trait;
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
// use chrono::{DateTime, NaiveDateTime, Utc};
// use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::{NoTls, Transaction};

#[async_trait]
pub trait CommunityRepository: Send + Sync {
    async fn create_community(
        &self,
        user_id: i32,
        community_req: &Community,
        tx: &mut Transaction<'_>,
    ) -> Result<(), CommunityError>;
}

pub struct CommunityRepositoryImpl {
    #[allow(dead_code)]
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl CommunityRepositoryImpl {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        CommunityRepositoryImpl { pool }
    }
}

#[async_trait]
impl CommunityRepository for CommunityRepositoryImpl {
    async fn create_community(
        &self,
        user_id: i32,
        community_req: &Community,
        tx: &mut Transaction<'_>,
    ) -> Result<(), CommunityError> {
        tx.execute(
            r#"
                INSERT INTO communities (
                    id,
                    name,
                    username,
                    profile_image,
                    bio,
                    created_by
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6
                )
            "#,
            &[
                &community_req.id,
                &community_req.name,
                &community_req.username,
                &community_req.profile_image,
                &community_req.bio,
                &user_id,
            ],
        )
        .await?;

        Ok(())
    }
}
