use std::time::SystemTime;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use tokio_postgres::{Transaction, NoTls};
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use crate::db::models::todo::{CompleteTodoItem, DeleteTodoItem, RequestCreateTodoItem, ResponseCreateTodoItem, TodoItem, UpdateTodoItem};
use crate::errors::todo_error::TodoError;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_todos(&self,
        user_id: String,
        tx: &mut Transaction<'_>
    ) -> Result<Vec<TodoItem>, TodoError>;

    async fn create_todo(
        &self,
        user_id: String,
        todo_req: &RequestCreateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<ResponseCreateTodoItem, TodoError>;

    async fn update_todo(
        &self,
        todo_req: &UpdateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError>;

    async fn delete_todo(
        &self,
        todo_req: &DeleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError>;
    
    async fn complete_todo(
        &self,
        todo_req: &CompleteTodoItem,
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
    async fn get_todos(
        &self,
        user_id: String,
        tx: &mut Transaction<'_>
    ) -> Result<Vec<TodoItem>, TodoError> {
        let rows = tx.query(
            r#"
            SELECT *
            FROM todos
            WHERE user_id = $1
            "#,
            &[&user_id],
        ).await?;

        let todos: Vec<TodoItem> = rows.into_iter().map(|row| {
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
                deadline: row.get::<_, Option<SystemTime>>("deadline")
                    .and_then(|time| NaiveDateTime::from_timestamp_opt(time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64, 0)),
                created_at: row.get::<_, Option<SystemTime>>("created_at")
                    .and_then(|time| NaiveDateTime::from_timestamp_opt(time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64, 0))
                    .expect("created_at must be NOT NULL"),
                updated_at: row.get::<_, Option<SystemTime>>("updated_at")
                    .and_then(|time| NaiveDateTime::from_timestamp_opt(time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64, 0))
                    .expect("updated_at must be NOT NULL"),
                deleted_at: row.get::<_, Option<SystemTime>>("deleted_at")
                    .and_then(|time| NaiveDateTime::from_timestamp_opt(time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64, 0)),
            }
        }).collect();

        Ok(todos)
    }

    async fn create_todo(
        &self,
        user_id: String,
        todo_req: &RequestCreateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<ResponseCreateTodoItem, TodoError> {
        let row = tx.query_one(
            r#"
                INSERT INTO todos (user_id, title, description, is_completed)
                VALUES ($1, $2, $3, false)
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

    async fn delete_todo(
        &self,
        todo_req: &DeleteTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<(), TodoError> {
        tx.execute(
            r#"
            UPDATE todos
            SET deleted_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
            &[&todo_req.id]
        ).await?;

        Ok(())
    }

    async fn update_todo(
        &self,
        todo_req: &UpdateTodoItem,
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

    async fn complete_todo(
        &self,
        todo_req: &CompleteTodoItem,
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
