//! タスクのサービスロジックで使用するカスタムエラー
//! 
//! * `DatabaseError`      - DB処理に関するエラー
//! * `PoolError`          - DB接続時に関するエラー
//! * `HashingError`       - ハッシュ化に関するエラー
//! * `TokenCreationError` - トークン作成に関するエラー
//! * `ValidationError`    - 入力値バリデーションに関するエラー
//! * `UserNotFound`       - ユーザーが見つからないエラー

use std::fmt;
use bb8_postgres::bb8;
use tokio_postgres;
use argon2;
use jsonwebtoken;

use super::user_error::UserError;

#[derive(Debug)]
pub enum TaskError {
    DatabaseError(String),
    PoolError(bb8::RunError<tokio_postgres::Error>),
    HashingError(argon2::password_hash::Error),
    TokenCreationError(jsonwebtoken::errors::Error),
    ValidationError(validator::ValidationErrors),
    UserNotFound,
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::DatabaseError(err) => write!(f, "Database connection error: {}", err),
            TaskError::PoolError(err) => write!(f, "Pool error: {}", err),
            TaskError::HashingError(err) => write!(f, "Password hashing error: {}", err),
            TaskError::TokenCreationError(err) => write!(f, "JWT error: {}", err),
            TaskError::ValidationError(err) => write!(f, "Validation error: {}", err),
            TaskError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl std::error::Error for TaskError {}

impl From<tokio_postgres::Error> for TaskError {
    fn from(error: tokio_postgres::Error) -> Self {
        TaskError::DatabaseError(error.to_string())
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for TaskError {
    fn from(error: bb8::RunError<tokio_postgres::Error>) -> Self {
        TaskError::PoolError(error)
    }
}

impl From<argon2::password_hash::Error> for TaskError {
    fn from(error: argon2::password_hash::Error) -> Self {
        TaskError::HashingError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for TaskError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        TaskError::TokenCreationError(error)
    }
}

impl From<UserError> for TaskError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::DatabaseError(err) => TaskError::DatabaseError(err.to_string()),
            UserError::PoolError(err) => TaskError::PoolError(err),
            UserError::HashingError(err) => TaskError::HashingError(err),
            UserError::TokenCreationError(err) => TaskError::TokenCreationError(err),
            UserError::ValidationError(err) => TaskError::ValidationError(err),
            UserError::UserNotFound => TaskError::UserNotFound,
        }
    }
}
