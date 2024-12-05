use std::fmt;
use bb8_postgres::bb8;
use tokio_postgres;
use argon2;
use jsonwebtoken;

use super::user_error::UserError;

#[derive(Debug)]
pub enum TodoError {
    DatabaseError(String),
    PoolError(bb8::RunError<tokio_postgres::Error>),
    HashingError(argon2::password_hash::Error),
    TokenCreationError(jsonwebtoken::errors::Error),
    UserNotFound
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::DatabaseError(err) => write!(f, "Database connection error: {}", err),
            TodoError::PoolError(err) => write!(f, "Pool error: {}", err),
            TodoError::HashingError(err) => write!(f, "Password hashing error: {}", err),
            TodoError::TokenCreationError(err) => write!(f, "JWT error: {}", err),
            TodoError::UserNotFound => write!(f, "user not found"),
        }
    }
}

impl std::error::Error for TodoError {}

impl From<tokio_postgres::Error> for TodoError {
    fn from(error: tokio_postgres::Error) -> Self {
        TodoError::DatabaseError(error.to_string())
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for TodoError {
    fn from(error: bb8::RunError<tokio_postgres::Error>) -> Self {
        TodoError::PoolError(error)
    }
}

impl From<argon2::password_hash::Error> for TodoError {
    fn from(error: argon2::password_hash::Error) -> Self {
        TodoError::HashingError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for TodoError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        TodoError::TokenCreationError(error)
    }
}

impl From<UserError> for TodoError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::DatabaseError(err) => TodoError::DatabaseError(err),
            UserError::PoolError(err) => TodoError::PoolError(err),
            UserError::HashingError(err) => TodoError::HashingError(err),
            UserError::TokenCreationError(err) => TodoError::TokenCreationError(err),
            UserError::UserNotFound => TodoError::UserNotFound,
        }
    }
}