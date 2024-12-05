//! # TODO ハンドラー
//!
//! ゲームトレーニングメニューのCRUD
//!
//! ## 関数
//!
//! - `get_todos`: TODO 取得
//! - `create_todo`: TODO 作成
//! - `update_todo`: TODO 更新
//! - `delete_todo`: TODO 削除
//! - `complete_todo`: TODO 完了

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::jwt::jwt;
use crate::application::states::app_state::AppState;
use crate::domain::entities::todo::{
    RequestCreateTodoItem,
    RequestUpdateTodoItem,
    RequestDeleteTodoItem,
    RequestCompleteTodoItem
};
use crate::{app_log, error_log};

/// TODO 取得
/// 
/// # 引数
/// 
/// * `req` - ヘッダーにトークンを含むリクエスト
/// * `app_state` - DIを含む状態
/// 
/// # 返却値
/// 
/// 認証ユーザーが持つ TODO を返却
pub async fn get_todos(
    req: HttpRequest,
    app_state: web::Data<AppState>
) -> impl Responder {
    let todo_service = &app_state.todo_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match todo_service.get_todos(user_data).await {
            Ok(todos) => {
                HttpResponse::Ok().json(todos)
            },
            Err(todo_error) => {
                error_log!("[todo_controller] - [get_todos] message: todo_error = {}", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [get_todos] message: error = {}", error);
            HttpResponse::Unauthorized().finish()
        }
    }
}

/// TODO 作成
/// 
/// # 引数
/// 
/// * `req` - ヘッダーにトークンを含むリクエスト
/// * `todo_req` - `RequestCreateTodoItem` 型の JSON
/// * `app_state` - DIを含む状態
/// 
/// # 返却値
/// 
/// 認証ユーザーが持つ TODO を返却
pub async fn create_todo(
    req: HttpRequest,
    todo_req: web::Json<RequestCreateTodoItem>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let todo_service = &app_state.todo_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match todo_service.create_todo(user_data, &todo_req).await {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(todo_error) => {
                error_log!("[todo_controller] - [create_todo] message: todo_error = {}", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [create_todo] message: error = {}", error);
            HttpResponse::Unauthorized().finish()
        }
    }
}

/// TODO 更新
/// 
/// # 引数
/// 
/// * `req` - ヘッダーにトークンを含むリクエスト
/// * `todo_req` - `RequestUpdateTodoItem` 型の JSON
/// * `app_state` - DIを含む状態
/// 
/// # 返却値
/// 
/// ステータスコード 200 を返却
pub async fn update_todo(
    req: HttpRequest,
    todo_req: web::Json<RequestUpdateTodoItem>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let todo_service = &app_state.todo_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match todo_service.update_todo(user_data, &todo_req).await {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(todo_error) => {
                error_log!("[todo_controller] - [update_todo] message: todo_error = {}", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [update_todo] message: error = {}", error);
            HttpResponse::Unauthorized().finish()
        }
    }
}

/// TODO 削除
/// 
/// # 引数
/// 
/// * `req` - ヘッダーにトークンを含むリクエスト
/// * `todo_req` - `RequestDeleteTodoItem` 型の JSON
/// * `app_state` - DIを含む状態
/// 
/// # 返却値
/// 
/// ステータスコード 200 を返却
pub async fn delete_todo(
    req: HttpRequest,
    todo_req: web::Json<RequestDeleteTodoItem>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let todo_service = &app_state.todo_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match todo_service.delete_todo(user_data, &todo_req).await {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(todo_error) => {
                error_log!("[todo_controller] - [delete_todo] message: todo_error = {}", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [delete_todo] message: error = {}", error);
            HttpResponse::Unauthorized().finish()
        }
    }
}

/// TODO 完了
/// 
/// # 引数
/// 
/// * `req` - ヘッダーにトークンを含むリクエスト
/// * `todo_req` - `RequestCompleteTodoItem` 型の JSON
/// * `app_state` - DIを含む状態
/// 
/// # 返却値
/// 
/// ステータスコード 200 を返却
pub async fn complete_todo(
    req: HttpRequest,
    todo_req: web::Json<RequestCompleteTodoItem>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let todo_service = &app_state.todo_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match todo_service.complete_todo(user_data, &todo_req).await {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(todo_error) => {
                error_log!("[todo_controller] - [complete_todo] message: todo_error = {}", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [complete_todo] message: error = {}", error);
            HttpResponse::Unauthorized().finish()
        }
    }
}