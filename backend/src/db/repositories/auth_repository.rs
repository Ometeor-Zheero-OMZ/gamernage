//! # Authentication Repository Module
//!
//! This module provides the implementation for handling authentication-related operations such as logging in and signing up users. 
//! It interfaces with a PostgreSQL database to manage user data and credentials.
//!
//! ## Overview
//!
//! The `AuthRepository` trait defines the methods for user authentication operations, including guest login, user signup, and user login. 
//! The `AuthRepositoryImpl` struct implements this trait and performs database operations using a connection pool.
//!
//! - `guest_login` - Authenticates a user based on their email and password, creating a JWT token if successful.
//! - `signup` - Registers a new user by inserting user details and hashed password into the database.
//! - `login` - Authenticates a user based on their name, email, and password, returning user details and a JWT token if successful.
//!
//! ## Dependencies
//!
//! This module relies on the following crates:
//!
//! - `async_trait` - Provides support for asynchronous traits.
//! - `bcrypt` - Used for password verification.
//! - `tokio_postgres` - PostgreSQL client for asynchronous operations.
//! - `bb8_postgres` - PostgreSQL connection pool for `tokio_postgres`.
//! - `argon2` - Password hashing and verification.
//! - `serde` - Serialization and deserialization (if used in `User` or request structs).
//! - `thiserror` - Error handling (if used in `AuthError`).
//!
//! Ensure these crates are added to your `Cargo.toml` file.
//!
//! ## Usage
//!
//! To use this module, you need to implement the `AuthRepository` trait in your application and initialize it with a `Pool<PostgresConnectionManager<NoTls>>`. 
//! Below is an example of how to use the `AuthRepositoryImpl` struct:
//!
//! ```rust
//! use bb8_postgres::PostgresConnectionManager;
//! use tokio_postgres::NoTls;
//! use bb8::Pool;
//! use crate::auth_repository::AuthRepositoryImpl;
//! use crate::db::models::auth::LoginRequest;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = PostgresConnectionManager::new(NoTls);
//!     let pool = Pool::builder().build(manager).await?;
//!     let auth_repo = AuthRepositoryImpl::new(pool);
//!
//!     let login_request = LoginRequest { email: "user@example.com".to_string(), password: "password".to_string() };
//!     let result = auth_repo.guest_login(&login_request).await?;
//!
//!     match result {
//!         Some(user) => println!("User logged in: {:?}", user),
//!         None => println!("Login failed"),
//!     }
//!     Ok(())
//! }
//! ```
//!
//! In this example, we create an instance of `AuthRepositoryImpl`, call the `guest_login` method with a login request, and handle the result.


use async_trait::async_trait;
use bcrypt::verify;
use tokio_postgres::{NoTls, Transaction};
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString
    },
    Argon2
};
use crate::{
    api::jwt::jwt,
    db::models::{auth::{LoginRequest, SignupRequest}, user::User},
    errors::auth_error::AuthError,
    libraries::logger
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn signup(&self, req: &SignupRequest, tx: &mut Transaction<'_>) -> Result<(), AuthError>;
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
}

pub struct AuthRepositoryImpl {
    pool: Pool<PostgresConnectionManager<NoTls>>
}

impl AuthRepositoryImpl {
    /// Creates a new instance of `AuthRepositoryImpl`.
    ///
    /// # Arguments
    ///
    /// * `pool` - The connection pool used to interact with the database.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `AuthRepositoryImpl` instance.
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        AuthRepositoryImpl { pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    /// Attempts to log in a guest user.
    ///
    /// # Arguments
    ///
    /// * `req` - The login request containing the user's email and password.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(User))` - Returns user details if the authentication is successful.
    /// * `Ok(None)` - Returns `None` if the user does not exist or authentication fails.
    /// * `Err(AuthError)` - Returns an error if something goes wrong during the process.
    async fn guest_login(
        &self,
        req: &LoginRequest,
    ) -> Result<Option<User>, AuthError> {
        let conn = self.pool.get().await?;

        let rows = conn.query(
            r#"
                SELECT 
                    users.id,
                    user_profiles.name,
                    user_profiles.email,
                    user_auth.password
                FROM
                    users
                INNER JOIN
                    user_auth
                ON
                    user_auth.user_id = users.id
                INNER JOIN
                    user_profiles
                ON
                    user_profiles.user_id = user_auth.user_id
                WHERE
                    email = $1;
            "#,
            &[&req.email]
        ).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let id: i32 = rows.get(0).unwrap().get("id");
        let password: String = rows.get(0).unwrap().get("password");

        if verify(&req.password, &password).is_err() {
            logger::log(logger::Header::ERROR, "[auth_repository] - [guest_login] - [Authentication Failed]");
            return Ok(None);
        }

        match jwt::create_token(&req.email, &id) {
            Ok(token) => {
                let user_data = User {
                    id,
                    name: req.name.clone(),
                    email: req.email.clone(),
                    token,
                };
                Ok(Some(user_data))
            }
            Err(err) => {
                logger::log(logger::Header::ERROR, &format!("[auth_repository] - [guest_login] err = {}", err));
                Err(AuthError::TokenCreationError(err))
            }
        }
    }

    /// Registers a new user in the system.
    ///
    /// # Arguments
    ///
    /// * `req` - The signup request containing user details.
    /// * `tx` - The database transaction used for the operation.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Returns `()` if the user was successfully created.
    /// * `Err(AuthError)` - Returns an error if something goes wrong during the process.
    async fn signup(
        &self,
        req: &SignupRequest,
        tx: &mut Transaction<'_>
    ) -> Result<(), AuthError> {
        let result_row = tx.query_one(
            r#"
                INSERT INTO users (
                    created_at,
                    updated_at
                ) VALUES (
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                )
                RETURNING id;
            "#,
            &[]
        ).await?;

        let user_id: i32 = result_row.get("id");

        tx.execute(
            r#"
                INSERT INTO user_profiles (
                    user_id,
                    name,
                    email,
                    created_at,
                    updated_at
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                );
            "#,
            &[&user_id, &req.name, &req.email]
        ).await?;

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = argon2.hash_password(&req.password.as_bytes(), &salt)?.to_string();

        tx.execute(
            r#"
                INSERT INTO user_auth (
                    user_id,
                    password,
                    created_at,
                    updated_at
                ) VALUES (
                    $1,
                    $2,
                    CURRENT_TIMESTAMP,
                    CURRENT_TIMESTAMP
                );
            "#,
            &[&user_id, &hashed_password]
        ).await?;

        Ok(())
    }

    /// Attempts to log in a user.
    ///
    /// # Arguments
    ///
    /// * `req` - The login request containing the user's name, email, and password.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(User))` - Returns user details if the authentication is successful.
    /// * `Ok(None)` - Returns `None` if the user does not exist or authentication fails.
    /// * `Err(AuthError)` - Returns an error if something goes wrong during the process.
    async fn login(
        &self,
        req: &LoginRequest
    ) -> Result<Option<User>, AuthError> {
        let conn = self.pool.get().await?;

        let rows = conn.query(
            r#"
                SELECT 
                    users.id,
                    user_profiles.name,
                    user_profiles.email,
                    user_auth.password
                FROM
                    users
                INNER JOIN
                    user_auth
                ON
                    user_auth.user_id = users.id
                INNER JOIN
                    user_profiles
                ON
                    user_profiles.user_id = user_auth.user_id
                WHERE
                    user_profiles.name = $1
                    AND user_profiles.email = $2;
            "#,
            &[&req.name, &req.email]
        ).await?;

        // Return None if no user is found
        if rows.is_empty() {
            return Ok(None);
        }

        let id: i32 = rows.get(0).unwrap().get("id");
        let name: String = rows.get(0).unwrap().get("name");
        let email: String = rows.get(0).unwrap().get("email");
        let hashed_password: String = rows.get(0).unwrap().get("password");

        // Initialize Argon2
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&hashed_password)?;

        // Return None if password does not match
        if argon2.verify_password(req.password.as_bytes(), &parsed_hash).is_err() {
            logger::log(logger::Header::ERROR, &format!("[auth_repository] - [login] [message: Authentication Failed]"));
            return Ok(None);
        };

        let token = jwt::create_token(&req.email, &id)?;

        let user_data = User {
            id,
            name,
            email,
            token,
        };

        Ok(Some(user_data))
    }
}