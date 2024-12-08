//! # タスクリポジトリ　インタフェース

use async_trait::async_trait;
use crate::domain::entities::todo::*;
use crate::application::errors::todo_error::TodoError;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_todos(&self,
        user_id: i32,
    ) -> Result<Vec<TodoItem>, TodoError>;

    async fn create_todo(
        &self,
        user_id: i32,
        todo_req: &RequestCreateTodoItem,
    ) -> Result<ResponseCreateTodoItem, TodoError>;

    async fn update_todo(
        &self,
        todo_req: &RequestUpdateTodoItem,

    ) -> Result<(), TodoError>;

    async fn delete_todo(
        &self,
        todo_req: &RequestDeleteTodoItem,

    ) -> Result<(), TodoError>;
    
    async fn complete_todo(
        &self,
        todo_req: &RequestCompleteTodoItem,

    ) -> Result<(), TodoError>;
}