use std::fmt;
use bb8_postgres::bb8;
use tokio_postgres;
use argon2;
use jsonwebtoken;

#[derive(Debug)]
pub enum UserError {
    DatabaseError(String),
    PoolError(bb8::RunError<tokio_postgres::Error>),
    HashingError(argon2::password_hash::Error),
    TokenCreationError(jsonwebtoken::errors::Error),
    UserNotFound,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::DatabaseError(err) => write!(f, "Database connection error: {}", err),
            UserError::PoolError(err) => write!(f, "Pool error: {}", err),
            UserError::HashingError(err) => write!(f, "Password hashing error: {}", err),
            UserError::TokenCreationError(err) => write!(f, "JWT error: {}", err),
            UserError::UserNotFound => write!(f, "user not found"),
        }
    }
}

impl std::error::Error for UserError {}

impl From<tokio_postgres::Error> for UserError {
    fn from(error: tokio_postgres::Error) -> Self {
        UserError::DatabaseError(error.to_string())
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

impl From<()> for UserError {
    fn from(_: ()) -> Self {
        UserError::UserNotFound
    }
}