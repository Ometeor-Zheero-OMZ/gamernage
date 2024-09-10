use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
use postgres::NoTls;
use reqwest::StatusCode;

use crate::{
    api::{
        jwt::jwt,
        services::user_service::get_user_id,
        utils::message::DB_MSG
    },
    libraries::{app_state::AppState, logger}
};
use crate::constants::custom_type::TodoRepositoryArc;
use crate::db::models::todo::{
    ResponseTodoList,
    RequestCreateTodoItem,
    UpdateTodoItem,
    DeleteTodoItem,
    CompleteTodoItem
};

/// todo データを取得
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// ユーザー情報に紐づく todo データを取得し返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
pub async fn get_todos(
    req: HttpRequest,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let todo_repository: &TodoRepositoryArc = &app_state.todo_repository;

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

    let mut tx = match conn.transaction().await {
        Ok(tx) => tx,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let user_id = match get_user_id(&user, &tx).await {
        Ok(user_id) => user_id,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::BadRequest().json(format!("{}", DB_MSG.get("USER_INFO_NOT_FOUND_MSG").unwrap_or(&"")))
        }
    };

    let result = todo_repository.get_todos(user_id, &mut tx).await;

    match result {
        Ok(todos) => {
            if let Err(e) = tx.commit().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_COMMIT_FAILURE_MSG").unwrap_or(&""), e.to_string()));
                return HttpResponse::InternalServerError().finish();
            }
            logger::log(logger::Header::SUCCESS, DB_MSG.get("FETCH_DATA_SUCCESS_MSG").unwrap_or(&""));
            return HttpResponse::Ok().json(ResponseTodoList { todos });
        }
        Err(err) => {
            if let Err(e) = tx.rollback().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_ROLLBACK_FAILURE_MSG").unwrap_or(&""), e.to_string()));
            }
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    }
}

/// todo データを作成
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター：　認証済みユーザー情報
/// * `todo_req` - 作成する todo データ
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// ユーザー情報に紐づく todo データを作成し、RETURNING値を返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
pub async fn create_todo(
    req: HttpRequest,
    todo_req: web::Json<RequestCreateTodoItem>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let todo_repository: &TodoRepositoryArc = &app_state.todo_repository;

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

    let mut tx = match conn.transaction().await {
        Ok(tx) => tx,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
    
    // ユーザーの存在チェック
    let user_id = match get_user_id(&user, &tx).await {
        Ok(user_id) => user_id,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::BadRequest().json(format!("{}", DB_MSG.get("USER_INFO_NOT_FOUND_MSG").unwrap_or(&"")))
        }
    };

    let result = todo_repository.create_todo(user_id, &todo_req, &mut tx).await;

    match result {
        Ok(new_todo) => {
            if let Err(e) = tx.commit().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_COMMIT_FAILURE_MSG").unwrap_or(&""), e.to_string()));
                return HttpResponse::InternalServerError().finish();
            }
            logger::log(logger::Header::SUCCESS, DB_MSG.get("CREATE_DATA_SUCCESS_MSG").unwrap_or(&""));
            return HttpResponse::Ok().json(new_todo);
        },
        Err(err) => {
            if let Err(e) = tx.rollback().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_ROLLBACK_FAILURE_MSG").unwrap_or(&""), e.to_string()));
            }
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}

/// todo データを更新
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター：　認証済みユーザー情報
/// * `todo_req` - 更新する todo データ
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// 200 ステータスコードを返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
pub async fn update_todo(
    req: HttpRequest,
    todo_req: web::Json<UpdateTodoItem>,
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
    let _user_id = match get_user_id(&user, &transaction).await {
        Ok(user_id) => user_id,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::BadRequest().json(format!("{}", DB_MSG.get("USER_INFO_NOT_FOUND_MSG").unwrap_or(&"")))
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
            if let Err(e) = transaction.commit().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_COMMIT_FAILURE_MSG").unwrap_or(&""), e.to_string()));
                return HttpResponse::InternalServerError().finish();
            }
            logger::log(logger::Header::SUCCESS, DB_MSG.get("UPDATE_DATA_SUCCESS_MSG").unwrap_or(&""));
            return HttpResponse::Ok().finish();
        },
        Err(err) => {
            if let Err(e) = transaction.rollback().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_ROLLBACK_FAILURE_MSG").unwrap_or(&""), e.to_string()));
            }
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    }
}

/// todo データを論理削除
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター：　認証済みユーザー情報
/// * `todo_req` - 削除する todo データの id
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// 200 ステータスコードを返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
pub async fn delete_todo(
    req: HttpRequest,
    todo_req: web::Json<DeleteTodoItem>,
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
    let _user_id = match get_user_id(&user, &transaction).await {
        Ok(user_id) => user_id,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::BadRequest().json(format!("{}", DB_MSG.get("USER_INFO_NOT_FOUND_MSG").unwrap_or(&"")))
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

    match rows_result {
        Ok(_result) => {
            if let Err(e) = transaction.commit().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_COMMIT_FAILURE_MSG").unwrap_or(&""), e.to_string()));
                return HttpResponse::InternalServerError().finish();
            }
            logger::log(logger::Header::SUCCESS, DB_MSG.get("DELETE_DATA_SUCCESS_MSG").unwrap_or(&""));
            return HttpResponse::Ok().finish();
        },
        Err(err) => {
            if let Err(e) = transaction.rollback().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_ROLLBACK_FAILURE_MSG").unwrap_or(&""), e.to_string()));
            }
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}

/// todo ステータスを更新
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター：　認証済みユーザー情報
/// * `todo_req` - 更新する todo データの user_id
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// 200 ステータスコードを返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
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
    let user_id = match get_user_id(&user, &transaction).await {
        Ok(user_id) => user_id,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::BadRequest().json(format!("{}", DB_MSG.get("USER_INFO_NOT_FOUND_MSG").unwrap_or(&"")))
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

    match rows_result {
        Ok(_result) => {
            if let Err(e) = transaction.commit().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_COMMIT_FAILURE_MSG").unwrap_or(&""), e.to_string()));
                return HttpResponse::InternalServerError().finish();
            }
            logger::log(logger::Header::SUCCESS, DB_MSG.get("UPDATE_DATA_SUCCESS_MSG").unwrap_or(&""));
            return HttpResponse::Ok().finish();
        },
        Err(err) => {
            if let Err(e) = transaction.rollback().await {
                logger::log(logger::Header::ERROR, &format!("{} {}", DB_MSG.get("TRANSACTION_ROLLBACK_FAILURE_MSG").unwrap_or(&""), e.to_string()));
            }
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    }
}