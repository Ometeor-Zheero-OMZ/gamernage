//! # Database Configuration Module
//!
//! This module provides functionality to retrieve and build the configuration required for
//! connecting to a PostgreSQL database using environment variables. It utilizes the `tokio_postgres`
//! library to create the configuration object that will be used to establish a database connection.
//!
//! ## Overview
//!
//! The `get_config` function reads environment variables to construct a `tokio_postgres::Config` object
//! which holds the necessary details for connecting to a PostgreSQL database. This configuration includes
//! the database host, user, password, and name. The function will panic if any of these required environment
//! variables are not set, ensuring that all necessary information is provided before attempting to connect
//! to the database.
//!
//! ## Dependencies
//!
//! This module relies on the following crate:
//! - `std::env`: Provides the `env` to get environment variables.
//! - `tokio_postgres`: Provides the `Config` struct used to specify connection parameters for PostgreSQL.
//!
//! ## Environment Variables
//!
//! The following environment variables must be set for this module to function correctly:
//! - `DATABASE_HOST`: The host address of the PostgreSQL database server.
//! - `DATABASE_USER`: The username used for authenticating with the database.
//! - `DATABASE_PASSWORD`: The password associated with the username for database access.
//! - `DATABASE_NAME`: The name of the database to which you want to connect.
//!
//! Ensure these environment variables are properly configured in your environment or configuration
//! files before running the application. Missing or incorrect values will result in a panic at runtime.
//!
//! # Example
//!
//! ```rust
//! let config = get_config();
//! ```

use std::env;

/// Returns the authenticated user database configuration.
///
/// This function reads the required environment variables to build the configuration for connecting
/// to a PostgreSQL database using `tokio_postgres::Config`.
///
/// # Arguments
/// This function does not take any arguments.
///
/// # Returns
/// * `tokio_postgres::Config` - Returns the PostgreSQL configuration with host, user, password, and database name.
///
/// # Panics
/// This function will panic if any of the following environment variables are not set:
/// * `DATABASE_HOST`
/// * `DATABASE_USER`
/// * `DATABASE_PASSWORD`
/// * `DATABASE_NAME`
///
/// Make sure to configure these environment variables correctly before running the application.
pub fn get_config() -> tokio_postgres::Config {
    let mut config = tokio_postgres::Config::new();
    config.host(
        &env::var("DATABASE_HOST").expect("環境変数 `DATABASE_HOST` は設定する必要があります。"),
    );
    config.user(
        &env::var("DATABASE_USER").expect("環境変数 `DATABASE_USER` は設定する必要があります。"),
    );
    config.password(
        &env::var("DATABASE_PASSWORD")
            .expect("環境変数 `DATABASE_PASSWORD` は設定する必要があります。"),
    );
    config.dbname(
        &env::var("DATABASE_NAME").expect("環境変数 `DATABASE_NAME` は設定する必要があります。"),
    );

    config
}
