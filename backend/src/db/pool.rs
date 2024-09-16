//! # Database Pool Module
//!
//! # Overview
//! 
//! This module provides functionality for creating a connection pool for PostgreSQL using `bb8_postgres` and `tokio_postgres`.
//! The connection pool is configured to manage connections to the PostgreSQL database and handle connection pooling efficiently.
//!
//! ## Dependencies
//! - `bb8_postgres`: Provides the connection manager and pooling for PostgreSQL connections.
//! - `tokio_postgres`: Provides the PostgreSQL client and configuration utilities.
//!
//! ## Configuration
//! The PostgreSQL connection settings are retrieved from environment variables through the `get_config` function in the `config` module.

use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use tokio_postgres::NoTls;

/// Asynchronously creates and returns a connection pool for PostgreSQL.
///
/// This function retrieves the database configuration from the `get_config` function in the `config` module.
/// It creates a `PostgresConnectionManager` using the retrieved configuration and builds a connection pool with a maximum size of 100 connections.
///
/// # Returns
/// * `Pool<PostgresConnectionManager<NoTls>>` - A connection pool configured to manage PostgreSQL connections with TLS disabled.
///
/// # Panics
/// This function will panic if the connection pool fails to build. Ensure that the configuration is correct and the database is accessible before calling this function.
///
/// # Example
/// ```no_run
/// let pool = get_db_pool().await;
/// ```
pub async fn get_db_pool () -> Pool<PostgresConnectionManager<NoTls>> {
    let config = super::config::get_config();
    let pg_mgr = PostgresConnectionManager::new(config, NoTls);
    let pool = Pool::builder().max_size(100).build(pg_mgr).await.unwrap();
    return pool;
}