use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
use postgres::NoTls;
use reqwest::StatusCode;
use chrono::NaiveDateTime;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

use crate::{api::jwt::jwt, library::logger};
// use crate::api::model::todo::TodoItem;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTodoItem {
    pub user_id: i32
}

#[derive(Serialize, Debug)]
pub struct ResponseTodoList {
    todos: Vec<TodoItem>,
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

pub async fn get_todos(
    req: HttpRequest,
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

    let rows_result = transaction.query(
        r#"
        SELECT
            *
        FROM
            todos
        WHERE
            user_id = $1
        "#,
        &[&user.id]
    ).await;

    match rows_result {
        Ok(rows) => {
            let todos: Vec<TodoItem> = rows.into_iter().map(|row| {       
                let id: i32 = row.get("id");
                let user_id: Option<i32> = row.get("user_id");
                let game_id: Option<i32> = row.get("game_id");
                let title: String = row.get("title");
                let description: String = row.get("description");
                let is_completed: bool = row.get("is_completed");
                let status: Option<i32> = row.get("status");
                let priority: Option<i32> =  row.get("priority");
                let difficulty: Option<i32> =  row.get("difficulty");
                let deadline: Option<NaiveDateTime> = row.get::<_, Option<SystemTime>>("deadline")
                    .and_then(|time| {
                        NaiveDateTime::from_timestamp_opt(
                            time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                            0,
                        )
                    });
                let created_at: NaiveDateTime = row.get::<_, Option<SystemTime>>("created_at")
                    .and_then(|time| {
                        NaiveDateTime::from_timestamp_opt(
                            time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                            0,
                        )
                    })
                    .expect("created_at は NOT NULL です。");
                let updated_at: NaiveDateTime = row.get::<_, Option<SystemTime>>("updated_at")
                    .and_then(|time| {
                        NaiveDateTime::from_timestamp_opt(
                            time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                            0,
                        )
                    })
                    .expect("updated_at は NOT NULL です。");
                let deleted_at: Option<NaiveDateTime> = row.get::<_, Option<SystemTime>>("deleted_at")
                    .and_then(|time| {
                        NaiveDateTime::from_timestamp_opt(
                            time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                            0,
                        )
                    });

                TodoItem {
                    id,
                    user_id,
                    game_id,
                    title,
                    description,
                    is_completed,
                    status,
                    priority,
                    difficulty,
                    deadline,
                    created_at,
                    updated_at,
                    deleted_at,
                }
            }).collect();

            if let Err(e) = transaction.commit().await {
                logger::log(logger::Header::ERROR, &format!("トランザクションをコミット中にエラーが発生しました： {}", e.to_string()));
                return HttpResponse::InternalServerError().finish();
            }

            return HttpResponse::Ok().json(ResponseTodoList { todos });
        }
        Err(err) => {
            // transaction.rollback().await.unwrap();
            if let Err(e) = transaction.rollback().await {
                logger::log(logger::Header::ERROR, &format!("トランザクションをロールバック中にエラーが発生しました： {}", e.to_string()));
            }
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    }
}

/// TODO作成
pub async fn create_todo(
    req: HttpRequest,
    todo_req: web::Json<RequestCreateTodoItem>,
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
    
    let new_todo = match transaction.query_one(
        r#"
        INSERT INTO todos (user_id, title, description, is_completed)
        VALUES ($1, $2, $3, false)
        RETURNING *
        "#,
        &[
            &user.id,
            &todo_req.title,
            &todo_req.description,
        ]
    )
    .await {
        Ok(row) => ResponseCreateTodoItem {
            title: row.get("title"),
            description: row.get("description"),
            is_completed: row.get("is_completed")
        },
        Err(err) => {
            transaction.rollback().await.unwrap();
            logger::log(logger::Header::ERROR, &err.to_string());

            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Err(e) = transaction.commit().await {
        logger::log(logger::Header::ERROR, &format!("トランザクションをコミット中にエラーが発生しました： {}", e.to_string()));
        return HttpResponse::InternalServerError().finish();
    }

    return HttpResponse::Ok().json(new_todo);
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
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        "#,
        &[
            &todo_req.id,
            &todo_req.title,
            &todo_req.description,
            &todo_req.is_completed,
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