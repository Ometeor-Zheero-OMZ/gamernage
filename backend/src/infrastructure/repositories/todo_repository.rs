//! # タスクリポジトリ
//! 
//! タスク処理を定義したリポジトリ
//! 
//! ## メソッド
//! 
/// `get_todos`     - ユーザーが持つタスク一覧を取得します。  
/// `create_todo`   - 新規タスクを作成します。  
/// `update_todo`   - 既存のタスクを更新します。  
/// `delete_todo`   - タスクを削除します。  
/// `complete_todo` - タスクを完了状態にします。

use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::NoTls;
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::{
    application::errors::todo_error::TodoError,
    domain::entities::todo::*,
    domain::repositories::todo_repository::TodoRepository,
};

pub struct TodoRepositoryImpl {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl TodoRepositoryImpl {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        TodoRepositoryImpl { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
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
    /// - `Ok(Some(Vec<TodoItem>)` - タスクを取得した場合、タスクリストを返します。
    /// - `Err(TodoError)`         - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn get_todos(
        &self,
        user_id: i32,
    ) -> Result<Vec<TodoItem>, TodoError> {
        let conn = self.pool.get().await?;

        let rows = conn.query(
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
            // データベースから抽出したタイムスタンプを YYYY-MM-dd HH:mm:ss フォーマットに変換
            let convert_timestamp = |time: SystemTime| -> NaiveDateTime {
                let duration = time.duration_since(UNIX_EPOCH).unwrap();
                DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0)
                    .unwrap()
                    .naive_utc()
            };

            let todo = TodoItem {
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
            };

            todo
        }).collect();

        Ok(todos)
    }

    /// タスク作成
    /// 
    /// タスクを新規作成します。
    /// 
    /// # 引数
    /// 
    /// * `user_id` - ユーザーID
    /// * `todo_req` - `RequestCreateTodoItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(Some(ResponseCreateTodoItem)` - タスクを作成した場合、作成したタスクを返します。
    /// - `Err(TodoError)`                  - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn create_todo(
        &self,
        user_id: i32,
        todo_req: &RequestCreateTodoItem,
    ) -> Result<ResponseCreateTodoItem, TodoError> {
        let conn = self.pool.get().await?;

        let row_result = conn.query_one(
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
    /// * `todo_req` - `RequestUpdateTodoItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(())`         - タスクを更新した場合。
    /// - `Err(TodoError)` - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn update_todo(
        &self,
        todo_req: &RequestUpdateTodoItem,
    ) -> Result<(), TodoError> {
        let conn = self.pool.get().await?;

        conn.execute(
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

    /// タスク削除
    /// 
    /// タスクを削除します。
    /// 
    /// # 引数
    /// 
    /// * `todo_req` - `RequestDeleteTodoItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(())`         - タスクを削除した場合。
    /// - `Err(TodoError)` - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn delete_todo(
        &self,
        todo_req: &RequestDeleteTodoItem,
    ) -> Result<(), TodoError> {
        let conn = self.pool.get().await?;

        conn.execute(
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

    /// タスク完了
    /// 
    /// タスクを完了します。
    /// 
    /// # 引数
    /// 
    /// * `todo_req` - `RequestCompleteTodoItem` 型のリクエストボディデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します：
    /// 
    /// - `Ok(())`         - タスクを完了した場合。
    /// - `Err(TodoError)` - データベース接続やクエリエラーが発生した場合、カスタムエラーを返します。
    async fn complete_todo(
        &self,
        todo_req: &RequestCompleteTodoItem,
    ) -> Result<(), TodoError> {
        let conn = self.pool.get().await?;

        conn.execute(
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