//! Message Manager Module
//!
//! This module provides a centralized place for storing and managing server, database, and environment variable related messages.
//! It uses `lazy_static` to create static instances of `HashMap`s for various types of messages that can be used throughout the application.

use lazy_static::lazy_static;
use std::collections::HashMap;

// Server messages
lazy_static! {
    pub static ref SVR_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("SVR_BUILD_SUCCESS_MSG", "🚀 Server build succeeded");
        map.insert("SVR_BUILD_FAILURE_MSG", "🔥 Server build failed.");
        map.insert(
            "TOKEN_NOT_FOUND_IN_REQUEST_HEADER_MSG",
            "Authentication token not found in request header.",
        );

        map
    };
}

// Database messages
lazy_static! {
    /// A static `HashMap` that holds database-related messages.
    ///
    /// # Messages
    ///
    /// * `DB_CONNECTION_SUCCESS_MSG` - Message indicating successful database connection.
    /// * `DB_CONNECTION_FAILURE_MSG` - Message indicating failure in database connection.
    /// * `TRANSACTION_COMMIT_FAILURE_MSG` - Message indicating an error occurred during transaction commit.
    /// * `TRANSACTION_ROLLBACK_FAILURE_MSG` - Message indicating an error occurred during transaction rollback.
    /// * `USER_INFO_NOT_FOUND_MSG` - Message indicating that user information does not exist.
    /// * `FETCH_DATA_SUCCESS_MSG` - Message indicating successful data retrieval.
    /// * `CREATE_DATA_SUCCESS_MSG` - Message indicating successful data creation.
    /// * `UPDATE_DATA_SUCCESS_MSG` - Message indicating successful data update.
    /// * `DELETE_DATA_SUCCESS_MSG` - Message indicating successful data deletion.
    pub static ref DB_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("DB_CONNECTION_SUCCESS_MSG", "✅ Database connection successful.");
        map.insert("DB_CONNECTION_FAILURE_MSG", "🔥 Database connection failed.");
        map.insert("TRANSACTION_COMMIT_FAILURE_MSG", "Error occurred while committing the transaction:");
        map.insert("TRANSACTION_ROLLBACK_FAILURE_MSG", "Error occurred while rolling back the transaction:");

        // User information
        map.insert("USER_INFO_NOT_FOUND_MSG", "User information not found.");

        // CRUD operations
        map.insert("FETCH_DATA_SUCCESS_MSG", "Data retrieval successful.");
        map.insert("CREATE_DATA_SUCCESS_MSG", "Data creation successful.");
        map.insert("UPDATE_DATA_SUCCESS_MSG", "Data update successful.");
        map.insert("DELETE_DATA_SUCCESS_MSG", "Data deletion successful.");

        map
    };
}

// Environment variable messages
lazy_static! {
    /// A static `HashMap` that holds messages related to unset environment variables.
    ///
    /// # Messages
    ///
    /// * `NO_SET_ENV_VAR_FRONTEND_PORT` - Message indicating that the `FRONTEND_PORT` environment variable is not set.
    /// * `NO_SET_ENV_VAR_DATABASE_PORT` - Message indicating that the `DATABASE_PORT` environment variable is not set.
    /// * `NO_SET_ENV_VAR_DATABASE_URL` - Message indicating that the `DATABASE_URL` environment variable is not set.
    pub static ref SET_ENV_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("NO_SET_ENV_VAR_FRONTEND_PORT", "⚠️ Environment variable not set: FRONTEND_PORT");
        map.insert("NO_SET_ENV_VAR_DATABASE_PORT", "⚠️ Environment variable not set: DATABASE_PORT");
        map.insert("NO_SET_ENV_VAR_DATABASE_URL", "⚠️ Environment variable not set: DATABASE_URL");

        map
    };
}
