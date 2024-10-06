use serde::{Serialize, Deserialize};

use super::user::UserInfo;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Community {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub profile_image: String,
    pub bio: String,
    pub created_by: i32,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct FetchCommunityRequest {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommunityDetails {
    pub community: Community,
    pub members: Vec<UserInfo>
}