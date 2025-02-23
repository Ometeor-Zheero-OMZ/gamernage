//! # タスクリポジトリ　インタフェース

use async_trait::async_trait;
use crate::{
    application::errors::task_error::TaskError,
    domain::entities::task::*
};

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn get_tasks(&self,
        user_id: i32,
    ) -> Result<TaskListResponse, TaskError>;

    async fn create_task(
        &self,
        user_id: i32,
        task_req: &RequestCreateTaskItem,
    ) -> Result<ResponseCreateTaskItem, TaskError>;

    async fn update_task(
        &self,
        task_req: &RequestUpdateTaskItem,

    ) -> Result<(), TaskError>;

    async fn delete_task(
        &self,
        task_req: &RequestDeleteTaskItem,

    ) -> Result<(), TaskError>;
    
    async fn complete_task(
        &self,
        task_req: &RequestCompleteTaskItem,

    ) -> Result<(), TaskError>;
}