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

/// タスク一覧の取得
/// 
/// 認証されたユーザーのタスクリストを取得します。
/// 
/// # 引数
/// 
/// * `req`       - ヘッダーにJWTトークンを含むHTTPリクエスト
/// * `app_state` - アプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(todos)`             - タスクリストが正常に取得された場合、`タスクリストを返します。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `InternalServerError()` - サーバーエラーが発生した場合。
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

/// タスクの新規作成
/// 
/// 認証されたユーザーが新しいタスクを作成します。
/// 
/// # 引数
/// 
/// * `req`       - ヘッダーにJWTトークンを含むHTTPリクエスト
/// * `todo_req`  - `RequestCreateTodoItem` 型のJSON
/// * `app_state` - アプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(response)`          - タスクが正常に作成された場合、作成されたタスクの詳細を返します。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `InternalServerError()` - サーバーエラーが発生した場合。
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

/// タスクの更新
/// 
/// 認証されたユーザーが既存のタスクを更新します。
/// 
/// # 引数
/// 
/// * `req`       - ヘッダーにJWTトークンを含むHTTPリクエスト
/// * `todo_req`  - `RequestUpdateTodoItem` 型のJSON
/// * `app_state` - アプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(())`                - タスクが正常に更新された場合。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `InternalServerError()` - サーバーエラーが発生した場合。
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

/// タスクの削除
/// 
/// 既存のタスクを削除します。
/// 
/// # 引数
/// 
/// * `req`       - ヘッダーにJWTトークンを含むHTTPリクエスト
/// * `todo_req`  - `RequestDeleteTodoItem` 型のJSON
/// * `app_state` - アプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(())`                - タスクが正常に削除された場合。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `InternalServerError()` - サーバーエラーが発生した場合。
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

/// タスクの完了
/// 
/// 認証されたユーザーがタスクを完了済みにします。
/// 
/// # 引数
/// 
/// * `req`       - ヘッダーにJWTトークンを含むHTTPリクエスト
/// * `todo_req`  - `RequestCompleteTodoItem` 型のJSON
/// * `app_state` - DIを含むアプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(())`                - タスクが正常に完了した場合。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `InternalServerError()` - サーバーエラーが発生した場合。
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