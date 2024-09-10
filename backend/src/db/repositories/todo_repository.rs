use async_trait::async_trait;
use std::time::SystemTime;
use chrono::NaiveDateTime;
use tokio_postgres::{Transaction, Error, NoTls};
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};

use crate::db::models::todo::{TodoItem, RequestCreateTodoItem, ResponseCreateTodoItem};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get_todos(
        &self,
        user_id: String,
        tx: &mut Transaction<'_>
    ) -> Result<Vec<TodoItem>, Error>;

    async fn create_todo(
        &self,
        user_id: String,
        todo_req: &RequestCreateTodoItem,
        tx: &mut Transaction<'_>
    ) -> Result<ResponseCreateTodoItem, Error>;
}

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
    async fn get_todos(
        &self,
        user_id: String,
        tx: &mut Transaction<'_>
    ) -> Result<Vec<TodoItem>, Error> {
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
    ) -> Result<ResponseCreateTodoItem, Error> {
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
}