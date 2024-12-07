use std::sync::Arc;
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
use postgres::NoTls;

use crate::application::types::di_type::{AuthServiceArc, TodoServiceArc, UserServiceArc};
use crate::domain::services::auth_service::AuthServiceImpl;
use crate::domain::services::todo_service::TodoServiceImpl;
use crate::domain::repositories::todo_repository::TodoRepositoryImpl;
use crate::domain::services::user_service::UserServiceImpl;
use crate::infrastructure::repositories::auth_repository::AuthRepositoryImpl;
use crate::infrastructure::repositories::user_repository::UserRepositoryImpl;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthServiceArc,
    pub todo_service: TodoServiceArc,
    // ユーザー検索機能を実装予定のため dead_code は無視する
    pub user_service: UserServiceArc
}

impl AppState {
    pub fn init(pool: &Pool<PostgresConnectionManager<NoTls>>) -> AppState {
        let auth_repository= Arc::new(AuthRepositoryImpl::new(pool.clone()));
        let todo_repository= Arc::new(TodoRepositoryImpl::new(pool.clone()));
        let user_repository= Arc::new(UserRepositoryImpl::new(pool.clone()));
        let user_service = Arc::new(UserServiceImpl::new(user_repository.clone()));
        let auth_service= Arc::new(AuthServiceImpl::new(auth_repository.clone()));
        let todo_service= Arc::new(TodoServiceImpl::new(todo_repository.clone(), user_service.clone()));

        AppState {
            auth_service,
            todo_service,
            user_service
        }
    }
}
