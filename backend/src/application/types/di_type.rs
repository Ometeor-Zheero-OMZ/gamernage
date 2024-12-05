use std::sync::Arc;
use crate::domain::{repositories::{auth_repository::AuthRepository, todo_repository::TodoRepository, user_repository::UserRepository}, services::{auth_service::AuthService, todo_service::TodoService, user_service::UserService}};

pub type AuthServiceArc = Arc<dyn AuthService>;
pub type AuthRepositoryArc = Arc<dyn AuthRepository>;

pub type TodoServiceArc = Arc<dyn TodoService>;
pub type TodoRepositoryArc = Arc<dyn TodoRepository>;

pub type UserServiceArc = Arc<dyn UserService>;
pub type UserRepositoryArc = Arc<dyn UserRepository>;