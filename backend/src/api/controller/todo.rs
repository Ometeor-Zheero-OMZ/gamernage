use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
use postgres::NoTls;
use reqwest::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{api::jwt::jwt, library::logger};
// use crate::api::model::todo::TodoItem;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub id: i32,
    pub user_id: i32,
    pub game_id: i32,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub status: i32,
    pub priority: i32,
    pub difficulty: i32,
    pub deadline: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTodoItem {
    pub user_id: i32
}

#[derive(Serialize)]
pub struct TodoListResponse {
    todos: Vec<TodoItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodoItem {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTodoItem {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteTodoItem {
    pub id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteTodoItem {
    pub id: i32
}

pub async fn get_todos(
    req: HttpRequest,
    todo_req: web::Json<GetTodoItem>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    let _user = match jwt::verify(&req) {
        Ok(user_info) => user_info,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::new(StatusCode::UNAUTHORIZED);
        }
    };

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let transaction = match conn.transaction().await {
        Ok(tx) => tx,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let rows_result = transaction.query(
        r#"
        SELECT
            id, title, description, is_completed, created_at, updated_at
        FROM
            todos
        WHERE
            user_id = 1
        "#,
        &[&todo_req.user_id]
    ).await;

    match rows_result {
        Ok(rows) => {
            let todos: Vec<TodoItem> = rows.into_iter().map(|row| TodoItem {
                id: row.get("id"),
                user_id: row.get("user_id"),
                game_id: row.get("game_id"),
                title: row.get("title"),
                description: row.get("description"),
                is_completed: row.get("is_completed"),
                status: row.get("status"),
                priority: row.get("priority"),
                difficulty: row.get("difficulty"),
                deadline: row.get("deadline"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                deleted_at: row.get("deleted_at"),
            }).collect();

            transaction.commit().await.unwrap();
            
            HttpResponse::Ok().json(TodoListResponse { todos })
        }
        Err(err) => {
            transaction.rollback().await.unwrap();
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// TODO作成
pub async fn create_todo(
    req: HttpRequest,
    todo_req: web::Json<CreateTodoItem>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    let _user = match jwt::verify(&req) {
        Ok(user_info) => user_info,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::new(StatusCode::UNAUTHORIZED);
        }
    };

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let transaction = match conn.transaction().await {
        Ok(tx) => tx,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
    
    let not_completed = false;
    
    let rows_result = transaction.execute(
        r#"
        INSERT INTO todos (title, description, is_completed, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        &[
            &todo_req.title,
            &todo_req.description,
            &not_completed,
            &Utc::now(),
            &Utc::now(),
        ]
    ).await;

    match rows_result {
        Ok(_rows) => {
            transaction.commit().await.unwrap();
            return HttpResponse::Ok().finish();
        }
        Err(err) => {
            transaction.rollback().await.unwrap();
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}

/// TODO更新
pub async fn update_todo(
    _req: HttpRequest,
    todo_req: web::Json<UpdateTodoItem>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let transaction = match conn.transaction().await {
        Ok(tx) => tx,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let rows_result = transaction.execute(
        r#"
        UPDATE
            todos
        SET
            title = $2,
            description = $3,
            is_completed = $4,
            updated_at = $5
        WHERE id = $1
        "#,
        &[
            &todo_req.id,
            &todo_req.title,
            &todo_req.description,
            &todo_req.is_completed,
            &Utc::now(),
        ]
    ).await;

    match rows_result {
        Ok(_result) => {
            transaction.commit().await.unwrap();
            return HttpResponse::Ok().finish();
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    }
}

/// TODO削除
pub async fn delete_todo(
    _req: HttpRequest,
    todo_req: web::Json<DeleteTodoItem>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let transaction = match conn.transaction().await {
        Ok(tx) => tx,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let rows_result = transaction.execute(
        r#"
        UPDATE
            todos
        SET
            delete_at = now()
        WHERE
            todos.id = $1
        "#,
        &[
            &todo_req.id,
        ]
    ).await;

    transaction.commit().await.unwrap();

    match rows_result {
        Ok(_result) => {
            return HttpResponse::Ok().finish();
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}

/// タスク完了更新
pub async fn complete_todo(
    req: HttpRequest,
    todo_req: web::Json<CompleteTodoItem>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    let user = match jwt::verify(&req) {
        Ok(user_info) => user_info,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::new(StatusCode::UNAUTHORIZED);
        }
    };

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let transaction = match conn.transaction().await {
        Ok(tx) => tx,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
    
    // ユーザーの存在チェック
    let user_row_result = transaction.query_one(
        r#"
        SELECT id
        FROM users
        WHERE name = $1
        "#,
        &[&user.sub]
    ).await;
    let user_id: i32 = match user_row_result {
        Ok(user_row) => user_row.get("id"),
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::BadRequest().json(format!("ユーザー情報が存在しません。"))
        }
    };

    let rows_result = transaction.execute(
        r#"
        UPDATE
            todos
        SET
            deleted_at = now(),
            user_id = $2,
            is_completed = true
        WHERE
            users.id = $1
        ;
        "#,
        &[
            &todo_req.id,
            &user_id
        ]
    ).await;

    transaction.commit().await.unwrap();

    match rows_result {
        Ok(_result) => {
            return HttpResponse::Ok().finish();
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    }
}