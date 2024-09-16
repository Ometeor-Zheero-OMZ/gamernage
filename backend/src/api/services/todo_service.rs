//! Todo Service Module
//! 
//! This module provides an implementation of the `TodoService` trait for handling operations related to todo items.
//! It includes methods for retrieving, creating, updating, deleting, and completing todo items. The service interacts
//! with the todo repository and uses a PostgreSQL connection pool for database transactions.

use async_trait::async_trait;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use postgres::NoTls;
use std::sync::Arc;
use crate::{
    api::jwt::jwt::Claims,
    constants::custom_type::TodoRepositoryArc,
    db::models::todo::*,
    errors::todo_error::TodoError
};
use crate::libraries::logger;

use super::user_service::get_user_id;

#[async_trait]
pub trait TodoService: Send + Sync {
    async fn get_todos(&self, user: Claims) -> Result<Vec<TodoItem>, TodoError>;
    async fn create_todo(&self, user: Claims, todo_req: &RequestCreateTodoItem) -> Result<ResponseCreateTodoItem, TodoError>;
    async fn update_todo(&self, user: Claims, todo_req: &RequestUpdateTodoItem) -> Result<(), TodoError>;
    async fn delete_todo(&self, user: Claims, todo_req: &RequestDeleteTodoItem) -> Result<(), TodoError>;
    async fn complete_todo(&self, user: Claims, todo_req: &RequestCompleteTodoItem) -> Result<(), TodoError>;
}

/// Implementation of the `TodoService` trait.
pub struct TodoServiceImpl {
    /// The todo repository used for querying and updating todo data.
    todo_repository: TodoRepositoryArc,

    /// The PostgreSQL connection pool used for database transactions.
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>
}

impl TodoServiceImpl {
    /// Creates a new instance of `TodoServiceImpl`.
    /// 
    /// # Arguments
    /// 
    /// * `todo_repository` - The todo repository.
    /// * `pool` - The PostgreSQL connection pool.
    /// 
    /// # Returns
    /// 
    /// A new instance of `TodoServiceImpl`.
    pub fn new(todo_repository: TodoRepositoryArc, pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        TodoServiceImpl { todo_repository, pool: Arc::new(pool) }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    /// Retrieves all todo items for a given user.
    /// 
    /// # Arguments
    /// 
    /// * `user` - The authenticated user claims, including user ID.
    /// 
    /// # Returns
    /// 
    /// * `Ok(Vec<TodoItem>)` - If the retrieval is successful.
    /// * `Err(TodoError)` - If an error occurs during the operation.
    async fn get_todos(&self, user: Claims) -> Result<Vec<TodoItem>, TodoError> {
        let todo_repository = self.todo_repository.clone();

        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(TodoError::from)?;
        let mut tx = conn.transaction().await.map_err(TodoError::from)?;

        let result = async {
            let user_id = get_user_id(&user, &mut tx).await?;
            todo_repository.get_todos(user_id, &mut tx).await
        }.await;

        match result {
            Ok(value) => {
                tx.commit().await.map_err(TodoError::from)?;
                Ok(value)
            }
            Err(todo_error) => {
                tx.rollback().await.map_err(TodoError::from)?;
                logger::log(logger::Header::ERROR, &format!("[todo_service] - [get_todos] - todo_error = {}", todo_error));

                Err(todo_error)
            }
        }
    }

    /// Creates a new todo item for a given user.
    /// 
    /// # Arguments
    /// 
    /// * `user` - The authenticated user claims, including user ID.
    /// * `todo_req` - The request containing details of the todo item to be created.
    /// 
    /// # Returns
    /// 
    /// * `Ok(ResponseCreateTodoItem)` - If the creation is successful.
    /// * `Err(TodoError)` - If an error occurs during the operation.
    async fn create_todo(&self, user: Claims, todo_req: &RequestCreateTodoItem) -> Result<ResponseCreateTodoItem, TodoError> {
        let todo_repository = self.todo_repository.clone();

        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(TodoError::from)?;
        let mut tx = conn.transaction().await.map_err(TodoError::from)?;

        let result = async {
            let user_id = get_user_id(&user, &mut tx).await?;
            todo_repository.create_todo(user_id, &todo_req, &mut tx).await
        }.await;

        match result {
            Ok(value) => {
                tx.commit().await.map_err(TodoError::from)?;
                Ok(value)
            }
            Err(todo_error) => {
                tx.rollback().await.map_err(TodoError::from)?;
                logger::log(logger::Header::ERROR, &format!("[todo_service] - [create_todo] - todo_error = {}", todo_error));

                Err(todo_error)
            }
        }
    }

    /// Updates an existing todo item for a given user.
    /// 
    /// # Arguments
    /// 
    /// * `user` - The authenticated user claims, including user ID.
    /// * `todo_req` - The request containing updated details of the todo item.
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - If the update is successful.
    /// * `Err(TodoError)` - If an error occurs during the operation.
    async fn update_todo(&self, user: Claims, todo_req: &RequestUpdateTodoItem) -> Result<(), TodoError> {
        let todo_repository = self.todo_repository.clone();

        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(TodoError::from)?;
        let mut tx = conn.transaction().await.map_err(TodoError::from)?;

        let result = async {
            let _user_id = get_user_id(&user, &mut tx).await?;
            todo_repository.update_todo(&todo_req, &mut tx).await
        }.await;

        match result {
            Ok(_) => {
                tx.commit().await.map_err(TodoError::from)?;
                Ok(())
            }
            Err(todo_error) => {
                tx.rollback().await.map_err(TodoError::from)?;
                logger::log(logger::Header::ERROR, &format!("[todo_service] - [create_todo] - todo_error = {}", todo_error));

                Err(todo_error)
            }
        }
    }

    /// Deletes an existing todo item for a given user.
    /// 
    /// # Arguments
    /// 
    /// * `user` - The authenticated user claims, including user ID.
    /// * `todo_req` - The request containing details of the todo item to be deleted.
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - If the deletion is successful.
    /// * `Err(TodoError)` - If an error occurs during the operation.
    async fn delete_todo(&self, user: Claims, todo_req: &RequestDeleteTodoItem) -> Result<(), TodoError> {
        let todo_repository = self.todo_repository.clone();

        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(TodoError::from)?;
        let mut tx = conn.transaction().await.map_err(TodoError::from)?;

        let result = async {
            let _user_id = get_user_id(&user, &mut tx).await?;
            todo_repository.delete_todo(&todo_req, &mut tx).await
        }.await;

        match result {
            Ok(_) => {
                tx.commit().await.map_err(TodoError::from)?;
                Ok(())
            }
            Err(todo_error) => {
                tx.rollback().await.map_err(TodoError::from)?;
                logger::log(logger::Header::ERROR, &format!("[todo_service] - [create_todo] - todo_error = {}", todo_error));

                Err(todo_error)
            }
        }
    }

    /// Marks a todo item as complete for a given user.
    /// 
    /// # Arguments
    /// 
    /// * `user` - The authenticated user claims, including user ID.
    /// * `todo_req` - The request containing details of the todo item to be marked as complete.
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - If the operation is successful.
    /// * `Err(TodoError)` - If an error occurs during the operation.
    async fn complete_todo(&self, user: Claims, todo_req: &RequestCompleteTodoItem) -> Result<(), TodoError> {
        let todo_repository = self.todo_repository.clone();

        let pool = self.pool.clone();
        let mut conn = pool.get().await.map_err(TodoError::from)?;
        let mut tx = conn.transaction().await.map_err(TodoError::from)?;

        let result = async {
            let _user_id = get_user_id(&user, &mut tx).await?;
            todo_repository.complete_todo(&todo_req, &mut tx).await
        }.await;

        match result {
            Ok(_) => {
                tx.commit().await.map_err(TodoError::from)?;
                Ok(())
            }
            Err(todo_error) => {
                tx.rollback().await.map_err(TodoError::from)?;
                logger::log(logger::Header::ERROR, &format!("[todo_service] - [create_todo] - todo_error = {}", todo_error));

                Err(todo_error)
            }
        }
    }
}
