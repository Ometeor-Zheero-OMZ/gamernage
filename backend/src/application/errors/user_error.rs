//! ユーザーのサービスロジックで使用するカスタムエラー
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

#[derive(Debug)]
pub enum UserError {
    DatabaseError(tokio_postgres::Error),
    PoolError(bb8::RunError<tokio_postgres::Error>),
    HashingError(argon2::password_hash::Error),
    TokenCreationError(jsonwebtoken::errors::Error),
    ValidationError(validator::ValidationErrors),
    UserNotFound,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::DatabaseError(err) => write!(f, "Database connection error: {}", err),
            UserError::PoolError(err) => write!(f, "Pool error: {}", err),
            UserError::HashingError(err) => write!(f, "Password hashing error: {}", err),
            UserError::TokenCreationError(err) => write!(f, "JWT error: {}", err),
            UserError::ValidationError(err) => write!(f, "Validation error: {}", err),
            UserError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl std::error::Error for UserError {}

impl From<tokio_postgres::Error> for UserError {
    fn from(error: tokio_postgres::Error) -> Self {
        UserError::DatabaseError(error)
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for UserError {
    fn from(error: bb8::RunError<tokio_postgres::Error>) -> Self {
        UserError::PoolError(error)
    }
}

impl From<argon2::password_hash::Error> for UserError {
    fn from(error: argon2::password_hash::Error) -> Self {
        UserError::HashingError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for UserError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        UserError::TokenCreationError(error)
    }
}

impl From<validator::ValidationErrors> for UserError {
    fn from(error: validator::ValidationErrors) -> Self {
        UserError::ValidationError(error)
    }
}

impl From<()> for UserError {
    fn from(_: ()) -> Self {
        UserError::UserNotFound
    }
}