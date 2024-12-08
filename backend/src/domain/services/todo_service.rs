//! # タスクサービス
//! 
//! タスク処理を定義したサービス
//! 
//! ## メソッド
//! 
//! `get_todos`     - タスク一覧取得
//! `create_todo`   - タスク作成
//! `update_todo`   - タスク更新
//! `delete_todo`   - タスク削除
//! `complete_todo` - タスク完了

use async_trait::async_trait;
use crate::application::types::di_type::UserServiceArc;
use crate::{
    application::errors::todo_error::TodoError,
    application::jwt::jwt::Claims,
    application::types::di_type::TodoRepositoryArc,
    domain::entities::todo::*,
    {app_log, error_log}
};

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
    /// - `Ok(Vec<TodoItem>)` - タスクが取得できた場合、タスクのリストを返します。
    /// - `Err(TodoError)`    - タスク取得処理中にエラーが発生した場合、カスタムエラーを返します。
    async fn get_todos(&self, user: Claims) -> Result<Vec<TodoItem>, TodoError> {
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        let user_id = user_service.get_user_id(&user).await.map_err(TodoError::from)?;
        todo_repository.get_todos(user_id).await
    }

    /// タスクの新規作成
    /// 
    /// ユーザーが新しいタスクを作成します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// * `todo_req` - `RequestCreateTodoItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(ResponseCreateTodoItem)` - タスクが作成された場合、作成されたタスクの情報を返します。
    /// - `Err(TodoError)`             - タスク作成中にエラーが発生した場合、カスタムエラーを返します。
    async fn create_todo(&self, user: Claims, todo_req: &RequestCreateTodoItem) -> Result<ResponseCreateTodoItem, TodoError> {
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        let user_id = user_service.get_user_id(&user).await.map_err(TodoError::from)?;
        todo_repository.create_todo(user_id, &todo_req).await
    }

    /// タスクの更新
    /// 
    /// 指定されたタスクを更新します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// * `todo_req` - `RequestUpdateTodoItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(())`                     - タスクが更新された場合。
    /// - `Err(TodoError)`             - タスク更新中にエラーが発生した場合、カスタムエラーを返します。
    async fn update_todo(&self, user: Claims, todo_req: &RequestUpdateTodoItem) -> Result<(), TodoError> {
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        if let Err(err) = user_service.get_user_id(&user).await.map_err(TodoError::from) {
            error_log!("[todo_service] - [update_todo] - [message: Authentication Failed] - Error: {:?}", err);
            return Ok(());
        };
        todo_repository.update_todo(&todo_req).await
    }

    /// タスクの削除
    /// 
    /// 指定されたタスクを削除します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// * `todo_req` - `RequestDeleteTodoItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(())` - 正常にタスクが削除された場合。
    /// - `Err(TodoError)` - タスク削除中にエラーが発生した場合、カスタムエラーを返します。
    async fn delete_todo(&self, user: Claims, todo_req: &RequestDeleteTodoItem) -> Result<(), TodoError> {
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        if let Err(err) = user_service.get_user_id(&user).await.map_err(TodoError::from) {
            error_log!("[todo_service] - [delete_todo] - [message: Authentication Failed] - Error: {:?}", err);
            return Ok(());
        };

        todo_repository.delete_todo(&todo_req).await
    }

    /// タスクの完了
    /// 
    /// 指定されたタスクを完了状態に設定します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// * `todo_req` - `RequestCompleteTodoItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(())` - タスクが完了としてマークされた場合。
    /// - `Err(TodoError)` - タスク完了処理中にエラーが発生した場合、カスタムエラーを返します。
    async fn complete_todo(&self, user: Claims, todo_req: &RequestCompleteTodoItem) -> Result<(), TodoError> {
        let todo_repository = self.todo_repository.clone();
        let user_service = self.user_service.clone();

        if let Err(err) = user_service.get_user_id(&user).await.map_err(TodoError::from) {
            error_log!("[todo_service] - [complete_todo] - [message: Authentication Failed] - Error: {:?}", err);
            return Ok(());
        };

        todo_repository.complete_todo(&todo_req).await
    }
}