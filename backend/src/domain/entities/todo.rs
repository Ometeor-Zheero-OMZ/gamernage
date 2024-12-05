use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// TODO取得　リクエスト
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: i32,
    pub user_id: Option<i32>,
    pub game_id: Option<i32>,
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

// TODO一覧　レスポンス
#[derive(Serialize, Debug)]
pub struct ResponseTodoList {
    pub todos: Vec<TodoItem>,
}

/// TODO作成　リクエスト
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestCreateTodoItem {
    pub title: String,
    pub description: String,
}

// TODO作成　レスポンス
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ResponseCreateTodoItem {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

/// TODO更新　リクエスト
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestUpdateTodoItem {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
    pub updated_at: NaiveDateTime,
}

/// TODO更新　レスポンス
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResponseUpdateTodoItem {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
    pub updated_at: NaiveDateTime,
}

/// TODO削除　リクエスト
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestDeleteTodoItem {
    pub id: i32
}

/// TODO削除　レスポンス
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestCompleteTodoItem {
    pub id: i32
}