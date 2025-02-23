//! # 認証ハンドラー
//!
//! 未認証のユーザーのアクセスを許可しているハンドラー
//!
//! ## 関数
//!
//! `guest_login`  - ゲストログイン
//! `signup`       - 新規登録
//! `login`        - ログイン
//! `current_user` - 認証済みユーザーチェック

use actix_web::{HttpResponse, Responder, web};
use postgres::error::SqlState;
use serde_json::json;
use validator::Validate;
use crate::application::errors::auth_error::AuthError;
use crate::application::helpers::cookie::{clear_cookie, create_cookie};
use crate::application::states::app_state::AppState;
use crate::domain::entities::auth::{LoginRequest, SignupRequest};
use crate::{app_log, info_log, error_log, success_log};

pub async fn register_user(
    req: web::Json<SignupRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    info_log!("[auth_handler] - [register_user] register_user called");
    if let Err(_validation_errors) = req.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let auth_service = &app_state.auth_service;

    match auth_service.register_user(&req).await {
        Ok((signed_user, token)) => {
            // クッキー生成            
            let token_cookie = create_cookie(token);

            success_log!("[auth_controller] - [register_user] message: Signed up successfully");
            HttpResponse::Created().cookie(token_cookie).json(signed_user)
        }
        Err(AuthError::DatabaseError(ref error)) => {
            if let Some(db_error) = error.as_db_error() {
                if db_error.code() == &SqlState::UNIQUE_VIOLATION {
                    error_log!("[auth_controller] - [register_user] message: db_error = {}", db_error);
                    return HttpResponse::Conflict().finish();
                }
            }

            error_log!("[auth_controller] - [register_user] message: error = {}", error);
            HttpResponse::InternalServerError().finish()
        }
        Err(auth_error) => {
            error_log!("[auth_controller] - [register_user] message: auth_error = {}", auth_error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn login_user(
    req: web::Json<LoginRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    info_log!("[auth_handler] - [login_user] login_user called");
    if let Err(_validation_errors) = req.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let auth_service = &app_state.auth_service;

    info_log!("[auth_handler] - [login_user] before processing auth_service.login_user");
    match auth_service.login_user(&req).await {
        Ok((user_data, token)) => {
            // クッキー生成            
            let token_cookie = create_cookie(token);

            success_log!("[auth_controller] - [login_user] message: Logged in successfully");
            HttpResponse::Created().cookie(token_cookie).json(user_data)
        }
        Err(auth_error) => {
            error_log!("[auth_controller] - [login] message: auth_error = {}", auth_error);
            
            if matches!(auth_error, AuthError::UserNotFound) {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

pub async fn logout_user() -> impl Responder {
    info_log!("[auth_handler] - [logout_user] logout_user called");
    HttpResponse::Ok().cookie(clear_cookie()).json(json!({ "message": "User logged out"}))
}

