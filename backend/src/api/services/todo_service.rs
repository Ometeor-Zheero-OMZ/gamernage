use async_trait::async_trait;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use postgres::NoTls;
use std::sync::Arc;
use crate::{api::jwt::jwt::Claims, constants::custom_type::TodoRepositoryArc, db::models::todo::{
    RequestCompleteTodoItem, RequestDeleteTodoItem, RequestCreateTodoItem, ResponseCreateTodoItem, TodoItem, RequestUpdateTodoItem
}, errors::todo_error::TodoError};

use super::user_service::get_user_id;

#[async_trait]
pub trait TodoService: Send + Sync {
    async fn get_todos(&self, user: Claims) -> Result<Vec<TodoItem>, TodoError>;
    async fn create_todo(&self, user: Claims, todo_req: &RequestCreateTodoItem) -> Result<ResponseCreateTodoItem, TodoError>;
    async fn update_todo(&self, user: Claims, todo_req: &RequestUpdateTodoItem) -> Result<(), TodoError>;
    async fn delete_todo(&self, user: Claims, todo_req: &RequestDeleteTodoItem) -> Result<(), TodoError>;
    async fn complete_todo(&self, user: Claims, todo_req: &RequestCompleteTodoItem) -> Result<(), TodoError>;
}

pub struct TodoServiceImpl {
    todo_repository: TodoRepositoryArc,
    pool: Arc<Pool<PostgresConnectionManager<NoTls>>>
}

impl TodoServiceImpl {
    pub fn new(todo_repository: TodoRepositoryArc, pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        TodoServiceImpl { todo_repository, pool: Arc::new(pool) }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
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
            Err(e) => {
                tx.rollback().await.map_err(TodoError::from)?;
                Err(e)
            }
        }
    }
    
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
            Err(e) => {
                tx.rollback().await.map_err(TodoError::from)?;
                Err(e)
            }
        }
    }
    
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
            Err(e) => {
                tx.rollback().await.map_err(TodoError::from)?;
                Err(e)
            }
        }
    }
    
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
            Err(e) => {
                tx.rollback().await.map_err(TodoError::from)?;
                Err(e)
            }
        }
    }
    
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
            Err(e) => {
                tx.rollback().await.map_err(TodoError::from)?;
                Err(e)
            }
        }
    }
}
