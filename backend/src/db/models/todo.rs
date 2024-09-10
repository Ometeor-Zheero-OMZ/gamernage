use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub id: i32,
    pub user_id: Option<String>,
    pub game_id: Option<String>,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub status: Option<i32>,
    pub priority: Option<i32>,
    pub difficulty: Option<i32>,
    pub deadline: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Debug)]
pub struct ResponseTodoList {
    pub todos: Vec<TodoItem>,
}

#[derive(Serialize, Debug)]
pub struct ResponseCreateTodoItem {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateTodoItem {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTodoItem {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteTodoItem {
    pub id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteTodoItem {
    pub id: i32
}