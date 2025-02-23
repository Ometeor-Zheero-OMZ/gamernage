use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::domain::enums::task::{Priority, Status};

/// タスク取得　リクエスト
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TaskItem {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub due_date: NaiveDateTime,
    pub status: Option<Status>,
    pub completed: bool,
    pub priority: Option<Priority>,
    pub user_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// タスク一覧　リクエスト
#[derive(Deserialize)]
pub struct TaskListRequest {
    pub user_id: i32
}

#[derive(Serialize)]
/// タスク一覧　レスポンス
pub struct TaskListResponse {
    pub tasks: Vec<TaskItem>,
}

/// TODO一覧　レスポンス
#[derive(Serialize, Debug)]
pub struct ResponseTaskList {
    pub todos: Vec<TaskItem>,
}

/// TODO作成　リクエスト
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestCreateTaskItem {
    pub title: String,
    pub description: String,
}

// TODO作成　レスポンス
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ResponseCreateTaskItem {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

/// TODO更新　リクエスト
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestUpdateTaskItem {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
    pub updated_at: NaiveDateTime,
}

/// TODO更新　レスポンス
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResponseUpdateTaskItem {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
    pub updated_at: NaiveDateTime,
}

/// TODO削除　リクエスト
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestDeleteTaskItem {
    pub id: i32
}

/// TODO削除　レスポンス
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestCompleteTaskItem {
    pub id: i32
}