//! # アプリケーション状態
//! 
//! アプリケーション実行中に使用するサービス・リポジトリを管理
//! 各サービスは DI に基づいて初期化され、`AppState` を通じてアクセス可能

use std::sync::Arc;
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
use postgres::NoTls;
use crate::{
    application::types::di_type::{AuthServiceArc, TaskServiceArc, UserServiceArc},
    domain::services::auth_service::AuthServiceImpl,
    domain::services::task_service::TaskServiceImpl,
    domain::services::user_service::UserServiceImpl,
    infrastructure::repositories::auth_repository::AuthRepositoryImpl,
    infrastructure::repositories::task_repository::TaskRepositoryImpl,
    infrastructure::repositories::user_repository::UserRepositoryImpl
};

#[derive(Clone)]
pub struct AppState {
    /// 認証サービス
    pub auth_service: AuthServiceArc,

    /// タスク管理サービス
    pub task_service: TaskServiceArc,

    /// ユーザー管理サービス
    pub user_service: UserServiceArc
}

impl AppState {
    pub fn init(pool: &Pool<PostgresConnectionManager<NoTls>>) -> AppState {
        let auth_repository= Arc::new(AuthRepositoryImpl::new(pool.clone()));
        let task_repository= Arc::new(TaskRepositoryImpl::new(pool.clone()));
        let user_repository= Arc::new(UserRepositoryImpl::new(pool.clone()));
        let user_service = Arc::new(UserServiceImpl::new(user_repository.clone()));
        let auth_service= Arc::new(AuthServiceImpl::new(auth_repository.clone()));
        let task_service= Arc::new(TaskServiceImpl::new(task_repository.clone(), user_service.clone()));

        AppState {
            auth_service,
            task_service,
            user_service
        }
    }
}
