#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use crate::api::jwt::jwt::Claims;
    use crate::api::services::community_service::CommunityService;
    use crate::db::models::community::*;
    use crate::errors::community_error::CommunityError;
    use async_trait::async_trait;
    use mockall::mock;

    mock! {
        pub CommunityService {}
        #[async_trait]
        impl CommunityService for CommunityService{
            async fn create_community(&self, user: Claims, todo_req: &Community) -> Result<(), CommunityError>;
            async fn fetch_community_details(
                &self,
                user: Claims,
                community_id: i32,
            ) -> Result<CommunityDetails, CommunityError>;
        }
    }

    // コミュニティ作成
    #[actix_rt::test]
    async fn test_create_community() -> Result<(), CommunityError> {
        let mut mock_service = MockCommunityService::new();

        let request_create_community = Community {
            id: 1,
            name: "test community".to_string(),
            username: "test user".to_string(),
            profile_image: "865f8wef87e".to_string(),
            bio: "biography...".to_string(),
            created_by: 1,
        };

        let request_create_community_clone = request_create_community.clone();

        mock_service
            .expect_create_community()
            .returning(move |_, todo_req| {
                if todo_req == &request_create_community_clone {
                    Ok(())
                } else {
                    Err(CommunityError::DatabaseError("db error".to_string()))
                }
            });

        let user = Claims {
            id: 1,
            sub: "test_user".to_string(),
            exp: 1239,
        };

        let result = mock_service
            .create_community(user, &request_create_community)
            .await;

        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response, ());

        Ok(())
    }
}
