//! # タスクリポジトリ
//! 
//! タスク処理を定義したリポジトリ
//! 
//! ## メソッド
//! 
/// `get_tasks`     - ユーザーが持つタスク一覧を取得します。  
/// `create_task`   - 新規タスクを作成します。  
/// `update_task`   - 既存のタスクを更新します。  
/// `delete_task`   - タスクを削除します。  
/// `complete_task` - タスクを完了状態にします。

use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;
use tokio_postgres::NoTls;
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::domain::enums::task::Priority;
use crate::{
    application::errors::task_error::TaskError,
    domain::{entities::task::*, enums::task::Status, repositories::task_repository::TaskRepository},
};

pub struct TaskRepositoryImpl {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl TaskRepositoryImpl {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        TaskRepositoryImpl { pool }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    /// タスク一覧取得
    /// 
    /// ユーザーが持つタスクを取得します。
    /// 
    /// # 引数
    /// 
    /// * `user_id` - ユーザーID
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(Some(TaskListResponse)` - タスクを取得した場合、タスクリストを返します。
    /// - `Err(TaskError)`         - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn get_tasks(
        &self,
        user_id: i32,
    ) -> Result<TaskListResponse, TaskError> {
        let conn = self.pool.get().await?;

        let rows = conn.query(
            r#"
                SELECT
                    *
                FROM
                    tasks
                WHERE
                    user_id = $1
            "#,
            &[&user_id],
        ).await?;

        let tasks: TaskListResponse = TaskListResponse {
            tasks: rows.into_iter().map(|row| {
                // データベースから抽出したタイムスタンプを YYYY-MM-dd HH:mm:ss フォーマットに変換
                let convert_timestamp = |time: SystemTime| -> NaiveDateTime {
                    let duration = time.duration_since(UNIX_EPOCH).unwrap();
                    DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0)
                        .unwrap()
                        .naive_utc()
                };
        
                let status: Option<String> = row.get("status");
                let priority: Option<String> = row.get("priority");
        
                let task = TaskItem {
                    id: row.get("id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    due_date: row.get("due_date"),
                    status: status.and_then(|s| Status::from_str(&s).ok()),
                    completed: row.get("completed"),
                    priority: priority.and_then(|p| Priority::from_str(&p).ok()),
                    user_id: row.get("user_id"),
                    created_at: convert_timestamp(row.get("created_at")),
                    updated_at: convert_timestamp(row.get("updated_at")),
                };
        
                task
            }).collect(),
        };

        Ok(tasks)
    }

    /// タスク作成
    /// 
    /// タスクを新規作成します。
    /// 
    /// # 引数
    /// 
    /// * `user_id` - ユーザーID
    /// * `task_req` - `RequestCreateTaskItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(Some(ResponseCreateTaskItem)` - タスクを作成した場合、作成したタスクを返します。
    /// - `Err(TaskError)`                  - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn create_task(
        &self,
        user_id: i32,
        task_req: &RequestCreateTaskItem,
    ) -> Result<ResponseCreateTaskItem, TaskError> {
        let conn = self.pool.get().await?;

        let row_result = conn.query_one(
            r#"
                INSERT INTO tasks (
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
                &task_req.title,
                &task_req.description,
            ]
        ).await?;

        Ok(ResponseCreateTaskItem {
            title: row_result.get("title"),
            description: row_result.get("description"),
            is_completed: row_result.get("is_completed")
        })
    }

    /// タスク更新
    /// 
    /// タスクを更新します。
    /// 
    /// # 引数
    /// 
    /// * `task_req` - `RequestUpdateTaskItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(())`         - タスクを更新した場合。
    /// - `Err(TaskError)` - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn update_task(
        &self,
        task_req: &RequestUpdateTaskItem,
    ) -> Result<(), TaskError> {
        let conn = self.pool.get().await?;

        conn.execute(
            r#"
                UPDATE
                    tasks
                SET
                    title = $2,
                    description = $3,
                    is_completed = $4,
                    updated_at = CURRENT_TIMESTAMP
                WHERE
                    id = $1
            "#,
            &[
                &task_req.id,
                &task_req.title,
                &task_req.description,
                &task_req.is_completed
            ]
        ).await?;

        Ok(())
    }

    /// タスク削除
    /// 
    /// タスクを削除します。
    /// 
    /// # 引数
    /// 
    /// * `task_req` - `RequestDeleteTaskItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(())`         - タスクを削除した場合。
    /// - `Err(TaskError)` - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn delete_task(
        &self,
        task_req: &RequestDeleteTaskItem,
    ) -> Result<(), TaskError> {
        let conn = self.pool.get().await?;

        conn.execute(
            r#"
                UPDATE
                    tasks
                SET
                    deleted_at = CURRENT_TIMESTAMP
                WHERE
                    id = $1
            "#,
            &[&task_req.id]
        ).await?;

        Ok(())
    }

    /// タスク完了
    /// 
    /// タスクを完了します。
    /// 
    /// # 引数
    /// 
    /// * `task_req` - `RequestCompleteTaskItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(())`         - タスクを完了した場合。
    /// - `Err(TaskError)` - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn complete_task(
        &self,
        task_req: &RequestCompleteTaskItem,
    ) -> Result<(), TaskError> {
        let conn = self.pool.get().await?;

        conn.execute(
            r#"
                UPDATE
                    tasks
                SET
                    deleted_at = CURRENT_TIMESTAMP,
                    is_completed = true
                WHERE
                    id = $1
            "#,
            &[&task_req.id]
        ).await?;

        Ok(())
    }
}