use lambda_http::{Error as LambdaError, LambdaEvent};
use serde_json::{json, Value};
use postgres::error::SqlState;
use validator::Validate;
use crate::api::jwt::jwt;
use crate::db::models::auth::{LoginRequest, SignupRequest};
use crate::errors::auth_error::AuthError;
use crate::libraries::app_state::AppState;
use crate::{app_log, success_log, error_log};

// ゲストログイン
pub async fn guest_login(
    event: LambdaEvent<Value>, 
    app_state: &AppState
) -> Result<Value, LambdaError> {
    let body = event.payload;
    let req: LoginRequest = serde_json::from_value(body).map_err(|_| LambdaError::from("Invalid body"))?;

    // バリデーション
    if let Err(_validation_errors) = req.validate() {
        return Ok(json!({ "error": "Bad Request" }));
    }

    let auth_service = &app_state.auth_service;

    match auth_service.guest_login(&req).await {
        Ok(Some(user_data)) => Ok(json!(user_data)),
        Ok(None) => Ok(json!({ "error": "Unauthorized" })),
        Err(_) => Ok(json!({ "error": "Internal Server Error" })),
    }
}

// サインアップ
pub async fn signup(
    event: LambdaEvent<Value>, 
    app_state: &AppState
) -> Result<Value, LambdaError> {
    let body = event.payload;
    let req: SignupRequest = serde_json::from_value(body).map_err(|_| LambdaError::from("Invalid body"))?;

    // バリデーション
    if let Err(_validation_errors) = req.validate() {
        return Ok(json!({ "error": "Bad Request" }));
    }

    let auth_service = &app_state.auth_service;

    match auth_service.signup(&req).await {
        Ok(()) => {
            success_log!("[auth_handler] - [signup] - [message: Successfully signed up]");
            Ok(json!({ "message": "Successfully signed up" }))
        }
        Err(AuthError::DatabaseError(ref error)) => {
            if let Some(db_error) = error.as_db_error() {
                if db_error.code() == &SqlState::UNIQUE_VIOLATION {
                    error_log!("[auth_handler] - [signup] - [message: db_error = {}]", db_error);
                    return Ok(json!({ "error": "Conflict" }));
                }
            }

            error_log!("[auth_handler] - [signup] - [message: error = {}]", error);
            Ok(json!({ "error": "Internal Server Error" }))
        }
        Err(auth_error) => {
            error_log!("[auth_handler] - [signup] - [message: auth_error = {}]", auth_error);
            Ok(json!({ "error": "Internal Server Error" }))
        }
    }
}

// ログイン
pub async fn login(
    event: LambdaEvent<Value>, 
    app_state: &AppState
) -> Result<Value, LambdaError> {
    let body = event.payload;
    let req: LoginRequest = serde_json::from_value(body).map_err(|_| LambdaError::from("Invalid body"))?;

    // バリデーション
    if let Err(_validation_errors) = req.validate() {
        return Ok(json!({ "error": "Bad Request" }));
    }

    let auth_service = &app_state.auth_service;

    match auth_service.login(&req).await {
        Ok(Some(user_data)) => Ok(json!(user_data)),
        Ok(None) => {
            error_log!("[auth_handler] - [login] - [message: USER NOT FOUND]");
            Ok(json!({ "error": "Unauthorized" }))
        }
        Err(auth_error) => {
            error_log!("[auth_handler] - [login] - [message: auth_error = {}]", auth_error);
            Ok(json!({ "error": "Internal Server Error" }))
        }
    }
}

// 現在のユーザー
pub async fn current_user(
    event: LambdaEvent<Value>, 
) -> Result<Value, LambdaError> {
    match jwt::verify_from_lambda_event(event).await {
        Ok(user_info) => Ok(json!(user_info)),
        Err(error) => {
            error_log!("[auth_handler] - [current_user] - [message: error = {}]", error);
            Ok(json!({ "error": "Unauthorized" }))
        }
    }
    
}
