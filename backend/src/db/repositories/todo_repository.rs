//! # Todo Repository Module
//!
//! This module provides the implementation for managing "todo" items in a PostgreSQL database. 
//! It defines the `TodoRepository` trait and its implementation, `TodoRepositoryImpl`, 
//! which includes methods for CRUD operations and marking todos as complete.
//!
//! ## Overview
//!
//! The `TodoRepository` trait outlines the operations related to todo items:
//!
//! - `get_todos` - Retrieves a list of todo items for a specific user.
//! - `create_todo` - Creates a new todo item with the given details.
//! - `update_todo` - Updates the details of an existing todo item.
//! - `delete_todo` - Marks a todo item as deleted by setting the `deleted_at` timestamp.
//! - `complete_todo` - Marks a todo item as complete and sets the `deleted_at` timestamp.
//!
//! The `TodoRepositoryImpl` struct implements this trait using a PostgreSQL connection pool, 
//! handling the database interactions and error handling.
//!
//! ## Dependencies
//!
//! This module requires the following crates:
//!
//! - `async_trait` - Provides support for asynchronous traits.
//! - `chrono` - For handling date and time, including conversions between different time formats.
//! - `tokio_postgres` - Asynchronous PostgreSQL client for interacting with the database.
//! - `bb8_postgres` - PostgreSQL connection pool for managing database connections.
//! - `thiserror` - Provides custom error handling (if used in `TodoError`).
//!
//! Ensure these dependencies are added to your `Cargo.toml` file to use this module.
//!
//! ## Usage
//!
//! To use this module, implement the `TodoRepository` trait and create an instance of `TodoRepositoryImpl` with a connection pool. 
//! Here’s an example of how to use the `TodoRepositoryImpl` struct:
//!
//! ```rust
//! use bb8_postgres::PostgresConnectionManager;
//! use tokio_postgres::NoTls;
//! use bb8::Pool;
//! use crate::todo_repository::TodoRepositoryImpl;
//! use crate::db::models::todo::{RequestCreateTodoItem, TodoItem};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = PostgresConnectionManager::new(NoTls);
//!     let pool = Pool::builder().build(manager).await?;
//!     let todo_repo = TodoRepositoryImpl::new(pool);
//!
//!     // Example: Create a new todo item
//!     let create_req = RequestCreateTodoItem { title: "New Todo".to_string(), description: "Description here".to_string() };
//!     let mut tx = todo_repo.pool.get().await?.begin().await?;
//!     let response = todo_repo.create_todo(1, &create_req, &mut tx).await?;
//!     tx.commit().await?;
//!
//!     println!("Created Todo: {:?}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! In this example, we initialize the `TodoRepositoryImpl` with a PostgreSQL connection pool and perform a create operation. 
//! Adjust the code as needed for other operations and handling errors.

use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::{Transaction, NoTls};
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::db::models::todo::{RequestCompleteTodoItem, RequestDeleteTodoItem, RequestCreateTodoItem, ResponseCreateTodoItem, TodoItem, RequestUpdateTodoItem};
use crate::errors::todo_error::TodoError;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_todos(&self,
        user_id: i32,
        tx: &mut Transaction<'_>
    ) -> Result<Vec<TodoItem>, TodoError>;

    async fn create_todo(
        &self,
        user_id: i32,
        todo_req: &RequestCreateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<ResponseCreateTodoItem, TodoError>;

    async fn update_todo(
        &self,
        todo_req: &RequestUpdateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError>;

    async fn delete_todo(
        &self,
        todo_req: &RequestDeleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError>;
    
    async fn complete_todo(
        &self,
        todo_req: &RequestCompleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError>;
}

pub struct TodoRepositoryImpl {
    #[allow(dead_code)]
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl TodoRepositoryImpl {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        TodoRepositoryImpl { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    /// Retrieves a list of todo items for a specific user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose todo items are to be retrieved.
    /// * `tx` - A mutable reference to an active database transaction.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<TodoItem>)` - A vector of `TodoItem` objects if the operation succeeds.
    /// * `Err(TodoError)` - An error if the operation fails.
    ///
    /// # Errors
    ///
    /// This function may return an error if the query fails or if there is an issue with the database connection.
    async fn get_todos(
        &self,
        user_id: i32,
        tx: &mut Transaction<'_>
    ) -> Result<Vec<TodoItem>, TodoError> {
        let rows = tx.query(
            r#"
                SELECT
                    *
                FROM
                    todos
                WHERE
                    user_id = $1
            "#,
            &[&user_id],
        ).await?;

        let todos: Vec<TodoItem> = rows.into_iter().map(|row| {
            // Convert timestamp extracted from database into the format YYYY-MM-dd HH:mm:ss
            let convert_timestamp = |time: SystemTime| -> NaiveDateTime {
                let duration = time.duration_since(UNIX_EPOCH).unwrap();
                DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0)
                    .unwrap()
                    .naive_utc()
            };

            TodoItem {
                id: row.get("id"),
                user_id: row.get("user_id"),
                game_id: row.get("game_id"),
                title: row.get("title"),
                description: row.get("description"),
                is_completed: row.get("is_completed"),
                status: row.get("status"),
                priority: row.get("priority"),
                difficulty: row.get("difficulty"),
                deadline: row.get::<_, Option<SystemTime>>("deadline").map(convert_timestamp),
                created_at: convert_timestamp(row.get("created_at")),
                updated_at: convert_timestamp(row.get("updated_at")),
                deleted_at: row.get::<_, Option<SystemTime>>("deleted_at").map(convert_timestamp),
            }
        }).collect();

        Ok(todos)
    }

    /// Creates a new todo item in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user creating the todo item.
    /// * `todo_req` - The request object containing the details of the todo item to be created.
    /// * `tx` - A mutable reference to an active database transaction.
    ///
    /// # Returns
    ///
    /// * `Ok(ResponseCreateTodoItem)` - The created `TodoItem` details if the operation succeeds.
    /// * `Err(TodoError)` - An error if the operation fails.
    ///
    /// # Errors
    ///
    /// This function may return an error if the query fails or if there is an issue with the database connection.
    async fn create_todo(
        &self,
        user_id: i32,
        todo_req: &RequestCreateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<ResponseCreateTodoItem, TodoError> {
        let row = tx.query_one(
            r#"
                INSERT INTO todos (
                    user_id,
                    title,
                    description,
                    is_completed
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    false
                )
                RETURNING *
            "#,
            &[
                &user_id,
                &todo_req.title,
                &todo_req.description,
            ]
        ).await?;

        Ok(ResponseCreateTodoItem {
            title: row.get("title"),
            description: row.get("description"),
            is_completed: row.get("is_completed")
        })
    }

    /// Marks a todo item as deleted by setting the `deleted_at` timestamp.
    ///
    /// # Arguments
    ///
    /// * `todo_req` - The request object containing the ID of the todo item to be deleted.
    /// * `tx` - A mutable reference to an active database transaction.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - An empty result if the operation succeeds.
    /// * `Err(TodoError)` - An error if the operation fails.
    ///
    /// # Errors
    ///
    /// This function may return an error if the query fails or if there is an issue with the database connection.
    async fn delete_todo(
        &self,
        todo_req: &RequestDeleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError> {
        tx.execute(
            r#"
                UPDATE
                    todos
                SET
                    deleted_at = CURRENT_TIMESTAMP
                WHERE
                    id = $1
            "#,
            &[&todo_req.id]
        ).await?;

        Ok(())
    }

    /// Updates the details of an existing todo item.
    ///
    /// # Arguments
    ///
    /// * `todo_req` - The request object containing the updated details of the todo item.
    /// * `tx` - A mutable reference to an active database transaction.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - An empty result if the operation succeeds.
    /// * `Err(TodoError)` - An error if the operation fails.
    ///
    /// # Errors
    ///
    /// This function may return an error if the query fails or if there is an issue with the database connection.
    async fn update_todo(
        &self,
        todo_req: &RequestUpdateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError> {
        tx.execute(
            r#"
                UPDATE
                    todos
                SET
                    title = $2,
                    description = $3,
                    is_completed = $4,
                    updated_at = CURRENT_TIMESTAMP
                WHERE
                    id = $1
            "#,
            &[
                &todo_req.id,
                &todo_req.title,
                &todo_req.description,
                &todo_req.is_completed
            ]
        ).await?;

        Ok(())
    }

    /// Marks a todo item as complete and sets the `deleted_at` timestamp.
    ///
    /// # Arguments
    ///
    /// * `todo_req` - The request object containing the ID of the todo item to be marked as complete.
    /// * `tx` - A mutable reference to an active database transaction.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - An empty result if the operation succeeds.
    /// * `Err(TodoError)` - An error if the operation fails.
    ///
    /// # Errors
    ///
    /// This function may return an error if the query fails or if there is an issue with the database connection.
    async fn complete_todo(
        &self,
        todo_req: &RequestCompleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError> {
        tx.execute(
            r#"
                UPDATE
                    todos
                SET
                    deleted_at = CURRENT_TIMESTAMP,
                    is_completed = true
                WHERE
                    id = $1
            "#,
            &[&todo_req.id]
        ).await?;

        Ok(())
    }
}
