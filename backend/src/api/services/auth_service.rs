//! Authentication Service Module
//! 
//! This module provides an implementation of the `AuthService` trait for handling authentication-related operations.
//! It includes methods for guest login, user signup, and user login. The service interacts with the authentication
//! repository and uses a PostgreSQL connection pool for database transactions.

use std::sync::Arc;

use async_trait::async_trait;
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
use postgres::NoTls;
use validator::Validate;

use crate::constants::custom_type::AuthRepositoryArc;
use crate::db::models::{auth::{LoginRequest, SignupRequest}, user::User};
use crate::{app_log, error_log};
use crate::errors::auth_error::AuthError;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError>;
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
}

/// Implementation of the `AuthService` trait.
pub struct AuthServiceImpl {
    /// The authentication repository used for querying and updating authentication data.
    auth_repository: AuthRepositoryArc,

    /// The PostgreSQL connection pool used for database transactions.
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>
}

impl AuthServiceImpl {
    /// Creates a new instance of `AuthServiceImpl`.
    /// 
    /// # Arguments
    /// 
    /// * `auth_repository` - The authentication repository.
    /// * `pool` - The PostgreSQL connection pool.
    /// 
    /// # Returns
    /// 
    /// A new instance of `AuthServiceImpl`.
    pub fn new(auth_repository: AuthRepositoryArc, pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        AuthServiceImpl { auth_repository, pool: Arc::new(pool) }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    /// Handles guest login requests.
    /// 
    /// # Arguments
    /// 
    /// * `req` - The login request containing guest credentials.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(User))` - If the guest login is successful and a user is found.
    /// * `Ok(None)` - If the guest login is successful but no user is found.
    /// * `Err(AuthError)` - If an error occurs during the operation.
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError> {
        let user_opt = self.auth_repository.guest_login(req).await?;
        match user_opt {
            Some(user) => Ok(Some(user)),
            None => Ok(None)
        }
    }

    /// Handles user signup requests.
    /// 
    /// # Arguments
    /// 
    /// * `req` - The signup request containing user details.
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - If the signup operation is successful.
    /// * `Err(AuthError)` - If an error occurs during the operation, including validation errors.
    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError> {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(AuthError::from)?;
        let mut tx = conn.transaction().await.map_err(AuthError::from)?;

        let result = self.auth_repository.signup(req, &mut tx).await;

        match result {
            Ok(value) => {
                tx.commit().await.map_err(AuthError::from)?;
                Ok(value)
            }
            Err(auth_error) => {
                tx.rollback().await.map_err(AuthError::from)?;
                error_log!("[auth_service] - [signup] - [message: auth_error = {}]", auth_error);

                Err(auth_error)
            }
        }
    }

    /// Handles user login requests.
    /// 
    /// # Arguments
    /// 
    /// * `req` - The login request containing user credentials.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(User))` - If the login is successful and a user is found.
    /// * `Ok(None)` - If the login is successful but no user is found.
    /// * `Err(AuthError)` - If an error occurs during the operation, including validation errors.
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError> {
        if let Err(validation_errors) = req.validate() {
            return Err(AuthError::ValidationError(validation_errors));
        }

        self.auth_repository.login(req).await
    }
}