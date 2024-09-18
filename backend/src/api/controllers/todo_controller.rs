//! # TODO Controller
//!
//! This module provides controller functions related to managing TODO items.
//!
//! ## Functions
//!
//! - `get_todos`: Retrieves all TODO data for the authenticated user.
//! - `create_todo`: Creates a new TODO item.
//! - `update_todo`: Updates an existing TODO item.
//! - `delete_todo`: Soft deletes a TODO item.
//! - `complete_todo`: Marks a TODO item as completed.
//!
//! ## Dependencies
//!
//! - `actix_web`: Web application framework.
//! - `reqwest`: HTTP client library.
//! - `crate::api::jwt::jwt`: JWT creation and verification functions.
//! - `crate::db::models::todo::{RequestCreateTodoItem, RequestUpdateTodoItem, RequestDeleteTodoItem, RequestCompleteTodoItem}`: Models for TODO item requests.
//! - `crate::libraries::app_state::AppState`: Application state.


use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::StatusCode;
use crate::api::jwt::jwt;
use crate::{app_log, error_log};
use crate::libraries::app_state::AppState;
use crate::db::models::todo::{
    RequestCreateTodoItem,
    RequestUpdateTodoItem,
    RequestDeleteTodoItem,
    RequestCompleteTodoItem
};

/// Handles retrieval of all TODO items for the authenticated user.
/// 
/// # Arguments
/// 
/// * `req` - HTTP request object (`HttpRequest` type) containing request information.
/// * `app_state` - Application state service (`web::Data<AppState>` type) managing application state and dependencies.
/// 
/// # Returns
/// 
/// * `impl Responder` - Returns TODO data in JSON format if successful. Returns HTTP status code 401 UNAUTHORIZED if unauthorized.
/// 
/// # Example
/// 
/// ```rust
/// web::scope("/api")
///     .route("/todos", web::get().to(todo_controller::get_todos))
/// ```
/// 
/// This example routes GET requests to `/api/todos` to the `get_todos` function, which retrieves TODO data by `user_id`.
/// If successful, it returns the data with HTTP status code 200 OK.
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
                error_log!("[todo_controller] - [get_todos] - [message: todo_error = {}]", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [get_todos] - [message: error = {}]", error);
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Handles creation of a new TODO item.
/// 
/// # Arguments
/// 
/// * `req` - HTTP request object (`HttpRequest` type) containing request information.
/// * `todo_req` - Request body object (`web::Json<RequestCreateTodoItem>` type) containing TODO item details.
/// * `app_state` - Application state service (`web::Data<AppState>` type) managing application state and dependencies.
/// 
/// # Returns
/// 
/// * `impl Responder` - Returns the created TODO item data in JSON format if successful. Returns HTTP status code 401 UNAUTHORIZED if unauthorized.
/// 
/// # Example
/// 
/// ```rust
/// web::scope("/api")
///     .route("/todos", web::post().to(todo_controller::create_todo))
/// ```
/// 
/// This example routes POST requests to `/api/todos` to the `create_todo` function, which creates a new TODO item.
/// If successful, it returns the created TODO item data with HTTP status code 200 OK.
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
                error_log!("[todo_controller] - [create_todo] - [message: todo_error = {}]", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [create_todo] - [message: error = {}]", error);
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Updates an existing TODO item.
/// 
/// # Arguments
/// 
/// * `req` - HTTP request object (`HttpRequest` type) containing request information.
/// * `todo_req` - Request body object (`web::Json<RequestUpdateTodoItem>` type) containing updated TODO item details.
/// * `app_state` - Application state service (`web::Data<AppState>` type) managing application state and dependencies.
/// 
/// # Returns
/// 
/// * `impl Responder` - Returns HTTP status code 200 OK if successful. Returns HTTP status code 500 INTERNAL SERVER ERROR if an error occurs.
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
                error_log!("[todo_controller] - [update_todo] - [message: todo_error = {}]", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [update_todo] - [message: error = {}]", error);
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Soft deletes a TODO item.
/// 
/// # Arguments
/// 
/// * `req` - HTTP request object (`HttpRequest` type) containing request information.
/// * `todo_req` - Request body object (`web::Json<RequestDeleteTodoItem>` type) containing the ID of the TODO item to delete.
/// * `app_state` - Application state service (`web::Data<AppState>` type) managing application state and dependencies.
/// 
/// # Returns
/// 
/// * `impl Responder` - Returns HTTP status code 200 OK if successful. Returns HTTP status code 500 INTERNAL SERVER ERROR if an error occurs.
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
                error_log!("[todo_controller] - [delete_todo] - [message: todo_error = {}]", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [delete_todo] - [message: error = {}]", error);
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Marks a TODO item as completed.
/// 
/// # Arguments
/// 
/// * `req` - HTTP request object (`HttpRequest` type) containing request information.
/// * `todo_req` - Request body object (`web::Json<RequestCompleteTodoItem>` type) containing the ID of the TODO item to complete.
/// * `app_state` - Application state service (`web::Data<AppState>` type) managing application state and dependencies.
/// 
/// # Returns
/// 
/// * `impl Responder` - Returns HTTP status code 200 OK if successful. Returns HTTP status code 500 INTERNAL SERVER ERROR if an error occurs.
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
                error_log!("[todo_controller] - [complete_todo] - [message: todo_error = {}]", todo_error);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => {
            error_log!("[todo_controller] - [complete_todo] - [message: error = {}]", error);
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}