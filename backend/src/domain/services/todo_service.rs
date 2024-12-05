use async_trait::async_trait;

use crate::application::types::di_type::UserServiceArc;
use crate::{
    application::errors::todo_error::TodoError,
    application::jwt::jwt::Claims,
    application::types::di_type::TodoRepositoryArc,
    domain::entities::todo::*
};
use crate::{app_log, error_log, info_log};

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
    user_service: UserServiceArc,
}

impl TodoServiceImpl {
    pub fn new(todo_repository: TodoRepositoryArc, user_service: UserServiceArc) -> Self {
        TodoServiceImpl { todo_repository, user_service }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn get_todos(&self, user: Claims) -> Result<Vec<TodoItem>, TodoError> {
        info_log!("[service] get_todos called");
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        let user_id = user_service.get_user_id(&user).await.map_err(TodoError::from)?;
        info_log!("user_id = {}", user_id);
        todo_repository.get_todos(user_id).await
    }

    async fn create_todo(&self, user: Claims, todo_req: &RequestCreateTodoItem) -> Result<ResponseCreateTodoItem, TodoError> {
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        let user_id = user_service.get_user_id(&user).await.map_err(TodoError::from)?;
        todo_repository.create_todo(user_id, &todo_req).await
    }

    async fn update_todo(&self, user: Claims, todo_req: &RequestUpdateTodoItem) -> Result<(), TodoError> {
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        if let Err(err) = user_service.get_user_id(&user).await.map_err(TodoError::from) {
            error_log!("[todo_service] - [update_todo] - [message: Authentication Failed] - Error: {:?}", err);
            return Ok(());
        };
        todo_repository.update_todo(&todo_req).await
    }

    async fn delete_todo(&self, user: Claims, todo_req: &RequestDeleteTodoItem) -> Result<(), TodoError> {
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        if let Err(err) = user_service.get_user_id(&user).await.map_err(TodoError::from) {
            error_log!("[todo_service] - [update_todo] - [message: Authentication Failed] - Error: {:?}", err);
            return Ok(());
        };

        todo_repository.delete_todo(&todo_req).await
    }

    async fn complete_todo(&self, user: Claims, todo_req: &RequestCompleteTodoItem) -> Result<(), TodoError> {
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        if let Err(err) = user_service.get_user_id(&user).await.map_err(TodoError::from) {
            error_log!("[todo_service] - [update_todo] - [message: Authentication Failed] - Error: {:?}", err);
            return Ok(());
        };

        todo_repository.complete_todo(&todo_req).await
    }
}