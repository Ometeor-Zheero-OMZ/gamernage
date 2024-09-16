//! # Application State Module
//!
//! This module provides the application state (`AppState`) and its initialization for managing shared services.
//!
//! # Overview
//!
//! The `AppState` struct contains shared instances of services, which are used throughout the application. These services depend on a PostgreSQL connection pool for database access.
//!
//! ## Dependencies
//!
//! This module relies on the following crates and modules:
//!
//! - `bb8_postgres`: Provides the connection manager and pooling for PostgreSQL connections, allowing for efficient management of database connections.
//! - `tokio_postgres`: Provides the PostgreSQL client and configuration utilities. Used by `bb8_postgres` for connection management.
//! - `Arc`: A thread-safe reference-counted pointer used to share service instances across threads.
//! - `api::services::todo_service::TodoServiceImpl`: Implementation of the todo-related service.
//! - `api::services::auth_service::AuthServiceImpl`: Implementation of the authentication-related service.
//! - `db::repositories::auth_repository::AuthRepositoryImpl`: Implementation of the authentication repository for data access.
//! - `db::repositories::todo_repository::TodoRepositoryImpl`: Implementation of the todo repository for data access.
//!
//! ## Initialization
//! The `init` function is responsible for initializing the `AppState` by creating the necessary service instances and injecting the database pool into them.
//!
//! # Example
//!
//! ```rust
//! let app_state = AppState::init(&pool);
//! ```

use std::sync::Arc;
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
use postgres::NoTls;

use crate::api::services::todo_service::TodoServiceImpl;
use crate::constants::custom_type::{AuthRepositoryArc, AuthServiceArc, TodoRepositoryArc, TodoServiceArc};
use crate::db::repositories::{auth_repository::AuthRepositoryImpl, todo_repository::TodoRepositoryImpl};
use crate::api::services::auth_service::AuthServiceImpl;

/// `AppState` is the shared state of the application that holds
/// instances of services.
#[derive(Clone)]
pub struct AppState {
    /// Service for handling authentication-related logic.
    pub auth_service: AuthServiceArc,
    /// Service for handling todo-related logic.
    pub todo_service: TodoServiceArc,
}

impl AppState {
    /// Initializes the application state with services.
    ///
    /// # Arguments
    ///
    /// * `pool` - A connection pool to the PostgreSQL database.
    ///
    /// # Returns
    ///
    /// Returns an `AppState` containing initialized services.
    ///
    /// # Example
    ///
    /// ```
    /// let app_state = AppState::init(&pool);
    /// ```
    pub fn init(pool: &Pool<PostgresConnectionManager<NoTls>>) -> AppState {
        let auth_repository: AuthRepositoryArc = Arc::new(AuthRepositoryImpl::new(pool.clone()));
        let todo_repository: TodoRepositoryArc = Arc::new(TodoRepositoryImpl::new(pool.clone()));
        let auth_service: AuthServiceArc = Arc::new(AuthServiceImpl::new(auth_repository.clone(), pool.clone()));
        let todo_service: TodoServiceArc = Arc::new(TodoServiceImpl::new(todo_repository.clone(), pool.clone()));

        AppState {
            auth_service,
            todo_service,
        }
    }
}

