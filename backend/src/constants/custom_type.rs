//! Constants Types

use crate::{
    api::services::{
        auth_service::AuthService, community_service::CommunityService, todo_service::TodoService,
    },
    db::repositories::{
        auth_repository::AuthRepository, community_repository::CommunityRepository,
        todo_repository::TodoRepository,
    },
};
use std::sync::Arc;

pub type AuthServiceArc = Arc<dyn AuthService>;
pub type AuthRepositoryArc = Arc<dyn AuthRepository>;

pub type TodoServiceArc = Arc<dyn TodoService>;
pub type TodoRepositoryArc = Arc<dyn TodoRepository>;

pub type CommunityServiceArc = Arc<dyn CommunityService>;
pub type CommunityRepositoryArc = Arc<dyn CommunityRepository>;

// pub type UserServiceArc = Arc<dyn UserService>;
// pub type UserRepositoryArc = Arc<dyn UserRepository>;
