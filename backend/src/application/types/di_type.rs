//! 依存関係の型定義
//! 
//! 非同期トレイトおよびトレイトオブジェクトの型を定義

use std::sync::Arc;
use crate::domain::{repositories::{auth_repository::AuthRepository, todo_repository::TodoRepository, user_repository::UserRepository}, services::{auth_service::AuthService, todo_service::TodoService, user_service::UserService}};

// 認証
pub type AuthServiceArc = Arc<dyn AuthService>;
pub type AuthRepositoryArc = Arc<dyn AuthRepository>;
// タスク
pub type TodoServiceArc = Arc<dyn TodoService>;
pub type TodoRepositoryArc = Arc<dyn TodoRepository>;
// ユーザー
pub type UserServiceArc = Arc<dyn UserService>;
pub type UserRepositoryArc = Arc<dyn UserRepository>;