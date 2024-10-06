use crate::db::models::{community::Community, user::UserInfo};
use crate::errors::community_error::CommunityError;
use async_trait::async_trait;
use bb8_postgres::bb8::PooledConnection;
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

    async fn get_community_by_id(
        &self,
        community_id: i32,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    ) -> Result<Community, CommunityError>;

    async fn get_community_members(
        &self,
        community_id: i32,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>
    ) -> Result<Vec<UserInfo>, CommunityError>;
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

    async fn get_community_by_id(
        &self,
        community_id: i32,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    ) -> Result<Community, CommunityError> {
        let row = conn
            .query_one(
                r#"
                    SELECT
                        id,
                        name,
                        username,
                        profile_image,
                        bio,
                        created_by
                    FROM
                        communities
                    WHERE
                        id = $1
                "#,
                &[&community_id],
            )
            .await?;

        Ok(Community {
            id: row.get("id"),
            name: row.get("name"),
            username: row.get("username"),
            profile_image: row.get("profile_image"),
            bio: row.get("bio"),
            created_by: row.get("created_by"),
        })
    }

    async fn get_community_members(
        &self,
        community_id: i32,
        conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
    ) -> Result<Vec<UserInfo>, CommunityError> {
        let rows = conn
            .query(
                r#"
                    SELECT
                        user_profiles.user_id,
                        user_profiles.name,
                        user_profiles.username,
                        user_profiles.profile_image
                    FROM
                        user_profiles
                    JOIN
                        community
                    ON
                        community.user_id = user_profiles.user_id
                    WHERE
                        community.community_id = $1
                "#,
                &[&community_id],
            )
            .await?;

        let members = rows
            .iter()
            .map(|row| UserInfo {
                id: row.get("id"),
                name: row.get("name"),
                username: row.get("username"),
                profile_image: row.get("profile_image")
            })
            .collect();

        Ok(members)
    }
}
