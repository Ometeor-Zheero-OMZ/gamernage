//! Constants Types

use std::sync::Arc;
use crate::{api::services::{auth_service::AuthService, todo_service::TodoService}, db::repositories::{auth_repository::AuthRepository, todo_repository::TodoRepository}};

pub type AuthServiceArc = Arc<dyn AuthService>;
pub type AuthRepositoryArc = Arc<dyn AuthRepository>;

pub type TodoServiceArc = Arc<dyn TodoService>;
pub type TodoRepositoryArc = Arc<dyn TodoRepository>;