//! 依存関係の型定義
//! 
//! 非同期トレイトおよびトレイトオブジェクトの型を定義

use std::sync::Arc;
use crate::{
    domain::repositories::auth_repository::AuthRepository,
    domain::repositories::task_repository::TaskRepository,
    domain::repositories::user_repository::UserRepository,
    domain::services::auth_service::AuthService,
    domain::services::task_service::TaskService,
    domain::services::user_service::UserService
};

// 認証
pub type AuthServiceArc = Arc<dyn AuthService>;
pub type AuthRepositoryArc = Arc<dyn AuthRepository>;
// タスク
pub type TaskServiceArc = Arc<dyn TaskService>;
pub type TaskRepositoryArc = Arc<dyn TaskRepository>;
// ユーザー
pub type UserServiceArc = Arc<dyn UserService>;
pub type UserRepositoryArc = Arc<dyn UserRepository>;