//! 認証のサービスロジックで使用するカスタムエラー
//! 
//! * `DatabaseError`      - DB処理に関するエラー
//! * `PoolError`          - DB接続時に関するエラー
//! * `HashingError`       - ハッシュ化に関するエラー
//! * `TokenCreationError` - トークン作成に関するエラー
//! * `ValidationError`    - 入力値バリデーションに関するエラー

use std::fmt;
use bb8_postgres::bb8;
use tokio_postgres;
use argon2;
use jsonwebtoken;

#[derive(Debug)]
pub enum AuthError {
    DatabaseError(tokio_postgres::Error),
    PoolError(bb8::RunError<tokio_postgres::Error>),
    HashingError(argon2::password_hash::Error),
    TokenCreationError(jsonwebtoken::errors::Error),
    ValidationError(validator::ValidationErrors),
    UserNotFound,
    InvalidCredentials
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::DatabaseError(err) => write!(f, "Database connection error: {}", err),
            AuthError::PoolError(err) => write!(f, "Pool error: {}", err),
            AuthError::HashingError(err) => write!(f, "Password hashing error: {}", err),
            AuthError::TokenCreationError(err) => write!(f, "JWT error: {}", err),
            AuthError::ValidationError(err) => write!(f, "Validation error: {}", err),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials")
        }
    }
}

impl std::error::Error for AuthError {}

impl From<tokio_postgres::Error> for AuthError {
    fn from(error: tokio_postgres::Error) -> Self {
        AuthError::DatabaseError(error)
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for AuthError {
    fn from(error: bb8::RunError<tokio_postgres::Error>) -> Self {
        AuthError::PoolError(error)
    }
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(error: argon2::password_hash::Error) -> Self {
        AuthError::HashingError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        AuthError::TokenCreationError(error)
    }
}

impl From<validator::ValidationErrors> for AuthError {
    fn from(error: validator::ValidationErrors) -> Self {
        AuthError::ValidationError(error)
    }
}

impl From<()> for AuthError {
    fn from(_: ()) -> Self {
        AuthError::UserNotFound
    }
}