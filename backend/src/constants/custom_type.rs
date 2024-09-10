use std::sync::Arc;
use crate::db::repositories::{auth_repository::AuthRepository, todo_repository::TodoRepository};

pub type TodoRepositoryArc = Arc<dyn TodoRepository>;
pub type AuthRepositoryArc = Arc<dyn AuthRepository>;