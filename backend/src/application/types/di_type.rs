//! 依存関係の型定義
//! 
//! 非同期トレイトおよびトレイトオブジェクトの型を定義

use std::sync::Arc;
use crate::{
    domain::repositories::auth_repository::AuthRepository,
    domain::repositories::todo_repository::TodoRepository,
    domain::repositories::user_repository::UserRepository,
    domain::services::auth_service::AuthService,
    domain::services::todo_service::TodoService,
    domain::services::user_service::UserService
};

// 認証
pub type AuthServiceArc = Arc<dyn AuthService>;
pub type AuthRepositoryArc = Arc<dyn AuthRepository>;
// タスク
pub type TodoServiceArc = Arc<dyn TodoService>;
pub type TodoRepositoryArc = Arc<dyn TodoRepository>;
// ユーザー
pub type UserServiceArc = Arc<dyn UserService>;
pub type UserRepositoryArc = Arc<dyn UserRepository>;