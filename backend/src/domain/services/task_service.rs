//! # タスクサービス
//! 
//! タスク処理を定義したサービス
//! 
//! ## メソッド
//! 
//! `get_tasks`     - タスク一覧取得
//! `create_task`   - タスク作成
//! `update_task`   - タスク更新
//! `delete_task`   - タスク削除
//! `complete_task` - タスク完了

use async_trait::async_trait;
use crate::application::types::di_type::UserServiceArc;
use crate::{
    application::errors::task_error::TaskError,
    application::jwt::jwt::Claims,
    application::types::di_type::TaskRepositoryArc,
    domain::entities::task::*,
    {app_log, error_log}
};

#[async_trait]
pub trait TaskService: Send + Sync {
    async fn get_tasks(&self, user_id: i32) -> Result<TaskListResponse, TaskError>;
    async fn create_task(&self, user: Claims, task_req: &RequestCreateTaskItem) -> Result<ResponseCreateTaskItem, TaskError>;
    async fn update_task(&self, user: Claims, task_req: &RequestUpdateTaskItem) -> Result<(), TaskError>;
    async fn delete_task(&self, user: Claims, task_req: &RequestDeleteTaskItem) -> Result<(), TaskError>;
    async fn complete_task(&self, user: Claims, task_req: &RequestCompleteTaskItem) -> Result<(), TaskError>;
}

pub struct TaskServiceImpl {
    task_repository: TaskRepositoryArc,
    user_service: UserServiceArc,
}

impl TaskServiceImpl {
    pub fn new(task_repository: TaskRepositoryArc, user_service: UserServiceArc) -> Self {
        TaskServiceImpl { task_repository, user_service }
    }
}

#[async_trait]
impl TaskService for TaskServiceImpl {
    /// タスク一覧取得
    /// 
    /// ユーザーが持つ全てのタスクを取得します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(TaskListResponse)` - タスクが取得できた場合、タスクのリストを返します。
    /// - `Err(TaskError)`    - タスク取得処理中にエラーが発生した場合、カスタムエラーを返します。
    async fn get_tasks(&self, user_id: i32) -> Result<TaskListResponse, TaskError> {
        let task_repository = self.task_repository.clone();

        task_repository.get_tasks(user_id).await
    }

    /// タスクの新規作成
    /// 
    /// ユーザーが新しいタスクを作成します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// * `task_req` - `RequestCreateTaskItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(ResponseCreateTaskItem)` - タスクが作成された場合、作成されたタスクの情報を返します。
    /// - `Err(TaskError)`             - タスク作成中にエラーが発生した場合、カスタムエラーを返します。
    async fn create_task(&self, user: Claims, task_req: &RequestCreateTaskItem) -> Result<ResponseCreateTaskItem, TaskError> {
        let task_repository = self.task_repository.clone();
        let user_service = self.user_service.clone();

        let user_id = user_service.get_user_id(&user).await.map_err(TaskError::from)?;
        task_repository.create_task(user_id, &task_req).await
    }

    /// タスクの更新
    /// 
    /// 指定されたタスクを更新します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// * `task_req` - `RequestUpdateTaskItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(())`                     - タスクが更新された場合。
    /// - `Err(TaskError)`             - タスク更新中にエラーが発生した場合、カスタムエラーを返します。
    async fn update_task(&self, user: Claims, task_req: &RequestUpdateTaskItem) -> Result<(), TaskError> {
        let task_repository = self.task_repository.clone();
        let user_service = self.user_service.clone();

        if let Err(err) = user_service.get_user_id(&user).await.map_err(TaskError::from) {
            error_log!("[task_service] - [update_task] - [message: Authentication Failed] - Error: {:?}", err);
            return Ok(());
        };
        task_repository.update_task(&task_req).await
    }

    /// タスクの削除
    /// 
    /// 指定されたタスクを削除します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// * `task_req` - `RequestDeleteTaskItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(())` - 正常にタスクが削除された場合。
    /// - `Err(TaskError)` - タスク削除中にエラーが発生した場合、カスタムエラーを返します。
    async fn delete_task(&self, user: Claims, task_req: &RequestDeleteTaskItem) -> Result<(), TaskError> {
        let task_repository = self.task_repository.clone();
        let user_service = self.user_service.clone();

        if let Err(err) = user_service.get_user_id(&user).await.map_err(TaskError::from) {
            error_log!("[task_service] - [delete_task] - [message: Authentication Failed] - Error: {:?}", err);
            return Ok(());
        };

        task_repository.delete_task(&task_req).await
    }

    /// タスクの完了
    /// 
    /// 指定されたタスクを完了状態に設定します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// * `task_req` - `RequestCompleteTaskItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(())` - タスクが完了としてマークされた場合。
    /// - `Err(TaskError)` - タスク完了処理中にエラーが発生した場合、カスタムエラーを返します。
    async fn complete_task(&self, user: Claims, task_req: &RequestCompleteTaskItem) -> Result<(), TaskError> {
        let task_repository = self.task_repository.clone();
        let user_service = self.user_service.clone();

        if let Err(err) = user_service.get_user_id(&user).await.map_err(TaskError::from) {
            error_log!("[task_service] - [complete_task] - [message: Authentication Failed] - Error: {:?}", err);
            return Ok(());
        };

        task_repository.complete_task(&task_req).await
    }
}