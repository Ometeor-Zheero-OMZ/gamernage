use argon2;
use bb8_postgres::bb8;
use jsonwebtoken;
use std::fmt;
use tokio_postgres;

#[derive(Debug)]
pub enum CommunityError {
    DatabaseError(String),
    PoolError(bb8::RunError<tokio_postgres::Error>),
    HashingError(argon2::password_hash::Error),
    TokenCreationError(jsonwebtoken::errors::Error),
}

impl fmt::Display for CommunityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommunityError::DatabaseError(err) => write!(f, "Database connection error: {}", err),
            CommunityError::PoolError(err) => write!(f, "Pool error: {}", err),
            CommunityError::HashingError(err) => write!(f, "Password hashing error: {}", err),
            CommunityError::TokenCreationError(err) => write!(f, "JWT error: {}", err),
        }
    }
}

impl std::error::Error for CommunityError {}

impl From<tokio_postgres::Error> for CommunityError {
    fn from(error: tokio_postgres::Error) -> Self {
        CommunityError::DatabaseError(error.to_string())
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for CommunityError {
    fn from(error: bb8::RunError<tokio_postgres::Error>) -> Self {
        CommunityError::PoolError(error)
    }
}

impl From<argon2::password_hash::Error> for CommunityError {
    fn from(error: argon2::password_hash::Error) -> Self {
        CommunityError::HashingError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for CommunityError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        CommunityError::TokenCreationError(error)
    }
}
