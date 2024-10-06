use crate::api::jwt::jwt::Claims;
use crate::constants::custom_type::CommunityRepositoryArc;
use crate::db::models::community::{Community, CommunityDetails};
use crate::errors::community_error::CommunityError;
use crate::{app_log, error_log};
use async_trait::async_trait;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use postgres::NoTls;
use std::sync::Arc;

use super::user_service::get_user_id;

#[async_trait]
pub trait CommunityService: Send + Sync {
    async fn create_community(
        &self,
        user: Claims,
        community_req: &Community,
    ) -> Result<(), CommunityError>;

    async fn fetch_community_details(
        &self,
        user: Claims,
        community_id: i32,
    ) -> Result<CommunityDetails, CommunityError>;
}

pub struct CommunityServiceImpl {
    community_repository: CommunityRepositoryArc,
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,
}

impl CommunityServiceImpl {
    pub fn new(
        community_repository: CommunityRepositoryArc,
        pool: Pool<PostgresConnectionManager<NoTls>>,
    ) -> Self {
        CommunityServiceImpl {
            community_repository,
            pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl CommunityService for CommunityServiceImpl {
    async fn create_community(
        &self,
        user: Claims,
        community_req: &Community,
    ) -> Result<(), CommunityError> {
        let community_repository = self.community_repository.clone();

        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(CommunityError::from)?;
        
        let user_id = get_user_id(&user, &mut conn).await?;

        let mut tx = conn.transaction().await.map_err(CommunityError::from)?;

        let result = async {
            community_repository
                .create_community(user_id, community_req, &mut tx)
                .await
        }
        .await;

        match result {
            Ok(_value) => {
                tx.commit().await.map_err(CommunityError::from)?;
                Ok(())
            }
            Err(community_error) => {
                tx.rollback().await.map_err(CommunityError::from)?;
                error_log!(
                    "[community_service] - [create_community] - [message: community_error = {}]",
                    community_error
                );

                Err(community_error)
            }
        }
    }

    async fn fetch_community_details(
        &self,
        user: Claims,
        community_id: i32,
    ) -> Result<CommunityDetails, CommunityError> {
        let community_repository = self.community_repository.clone();

        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(CommunityError::from)?;

        let result = async {
            let _user_id = get_user_id(&user, &mut conn).await?;
            let community = community_repository.get_community_by_id(community_id, &mut conn).await?;
            let members = community_repository.get_community_members(community_id, &mut conn).await?;

            Ok(CommunityDetails { community, members })
        }
        .await;

        match result {
            Ok(value) => {
                Ok(value)
            },
            Err(community_error) => {
                error_log!(
                    "[community_service] - [create_community] - [message: community_error = {}]",
                    community_error
                );

                Err(community_error)
            }
        }
    } 
}
