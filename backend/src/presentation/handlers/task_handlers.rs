//! # TODO ハンドラー
//!
//! ゲームトレーニングメニューのCRUD
//!
//! ## 関数
//!
//! - `get_tasks`: TODO 取得
//! - `create_task`: TODO 作成
//! - `update_task`: TODO 更新
//! - `delete_task`: TODO 削除
//! - `complete_task`: TODO 完了

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::application::jwt::jwt;
use crate::application::states::app_state::AppState;
use crate::domain::entities::task::{
    RequestCompleteTaskItem, RequestCreateTaskItem, RequestDeleteTaskItem, RequestUpdateTaskItem, TaskListRequest
};
use crate::domain::entities::user::UserRequest;
use crate::{app_log, error_log, info_log};

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
/// - `Ok(tasks)`             - タスクリストが正常に取得された場合、`タスクリストを返します。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `NotFound`              - ユーザーが見つからない場合
/// - `InternalServerError()` - サーバーエラーが発生した場合。
pub async fn get_tasks(
    req: HttpRequest,
    // user_req: web::Json<UserRequest>,
    // task_req: web::Json<TaskListRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    info_log!("[task_controller] - [get_tasks] get_tasks called");

    let task_service = &app_state.task_service;
    let user_service = &app_state.user_service;

    // 認証処理
    let claims = match jwt::verify(&req) {
        Ok(claims) => claims,
        Err(error) => {
            error_log!("[task_controller] - [get_tasks] message: error = {}", error);
            return HttpResponse::Unauthorized().json(json!({ "message": "Not authorized, please login!"}))
        }
    };

    let user_req = UserRequest {
        user_id: claims.id.to_string(),
    };

    // ユーザー取得
    match user_service.find_user_by_id(&user_req).await {
        Ok(user) => user,
        Err(user_error) => {
            error_log!("[task_controller] - [get_tasks] message: user_error = {}", user_error);
            return HttpResponse::NotFound().finish();
        }
    };

    let task_req = TaskListRequest {
        user_id: claims.id,
    };

    // タスク一覧取得
    match task_service.get_tasks(task_req.user_id).await {
        Ok(tasks_response) => HttpResponse::Ok().json(tasks_response.tasks),
        Err(task_error) => {
            error_log!("[task_controller] - [get_tasks] message: task_error = {}", task_error);
            HttpResponse::InternalServerError().finish()
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
/// * `task_req`  - `RequestCreateTaskItem` 型のJSON
/// * `app_state` - アプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(response)`          - タスクが正常に作成された場合、作成されたタスクの詳細を返します。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `InternalServerError()` - サーバーエラーが発生した場合。
pub async fn create_task(
    req: HttpRequest,
    task_req: web::Json<RequestCreateTaskItem>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let task_service = &app_state.task_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match task_service.create_task(user_data, &task_req).await {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(task_error) => {
                error_log!("[task_controller] - [create_task] message: task_error = {}", task_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[task_controller] - [create_task] message: error = {}", error);
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
/// * `task_req`  - `RequestUpdateTaskItem` 型のJSON
/// * `app_state` - アプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(())`                - タスクが正常に更新された場合。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `InternalServerError()` - サーバーエラーが発生した場合。
pub async fn update_task(
    req: HttpRequest,
    task_req: web::Json<RequestUpdateTaskItem>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let task_service = &app_state.task_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match task_service.update_task(user_data, &task_req).await {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(task_error) => {
                error_log!("[task_controller] - [update_task] message: task_error = {}", task_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[task_controller] - [update_task] message: error = {}", error);
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
/// * `task_req`  - `RequestDeleteTaskItem` 型のJSON
/// * `app_state` - アプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(())`                - タスクが正常に削除された場合。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `InternalServerError()` - サーバーエラーが発生した場合。
pub async fn delete_task(
    req: HttpRequest,
    task_req: web::Json<RequestDeleteTaskItem>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let task_service = &app_state.task_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match task_service.delete_task(user_data, &task_req).await {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(task_error) => {
                error_log!("[task_controller] - [delete_task] message: task_error = {}", task_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[task_controller] - [delete_task] message: error = {}", error);
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
/// * `task_req`  - `RequestCompleteTaskItem` 型のJSON
/// * `app_state` - DIを含むアプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(())`                - タスクが正常に完了した場合。
/// - `Unauthorized()`        - ユーザーが認証されていない場合。
/// - `InternalServerError()` - サーバーエラーが発生した場合。
pub async fn complete_task(
    req: HttpRequest,
    task_req: web::Json<RequestCompleteTaskItem>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let task_service = &app_state.task_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match task_service.complete_task(user_data, &task_req).await {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(task_error) => {
                error_log!("[task_controller] - [complete_task] message: task_error = {}", task_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[task_controller] - [complete_task] message: error = {}", error);
            HttpResponse::Unauthorized().finish()
        }
    }
}