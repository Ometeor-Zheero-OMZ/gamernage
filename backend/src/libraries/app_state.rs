use crate::constants::custom_type::{AuthRepositoryArc, TodoRepositoryArc};

#[derive(Clone)]
pub struct AppState {
    pub todo_repository: TodoRepositoryArc,
    pub auth_repository: AuthRepositoryArc,
}