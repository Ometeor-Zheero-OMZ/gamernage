//! # Authentication Controller
//!
//! This module provides controller functions related to user authentication.
//!
//! ## Functions
//!
//! - `guest_login`: Handles guest login.
//! - `signup`: Handles user signup.
//! - `login`: Handles user login.
//! - `current_user`: Returns the currently authenticated user's data.
//!
//! ## Dependencies
//!
//! - `actix_web`: Web application framework.
//! - `postgres`: PostgreSQL database connection.
//! - `crate::api::jwt::jwt`: JWT creation and verification functions.
//! - `crate::db::models::auth::{LoginRequest, SignupRequest}`: Authentication request models.
//! - `crate::errors::auth_error::AuthError`: Authentication errors.
//! - `crate::libraries::app_state::AppState`: Application state.

use crate::api::jwt::jwt;
use crate::db::models::auth::{LoginRequest, SignupRequest};
use crate::errors::auth_error::AuthError;
use crate::libraries::app_state::AppState;
use crate::{app_log, error_log, success_log};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use postgres::error::SqlState;
use validator::Validate;

/// Handles guest login.
///
/// # Arguments
///
/// * `req` - JSON request data of type `LoginRequest`
/// * `app_state` - Application state
///
/// # Returns
///
/// Returns the authenticated user data if successful. Returns 401 if unauthorized.
pub async fn guest_login(
    req: web::Json<LoginRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    if let Err(_validation_errors) = req.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let auth_service = &app_state.auth_service;

    match auth_service.guest_login(&req).await {
        Ok(Some(user_data)) => HttpResponse::Ok().json(user_data),
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(_auth_error) => HttpResponse::InternalServerError().finish(),
    }
}

/// Handles user signup.
///
/// # Arguments
///
/// * `req` - JSON request data of type `SignupRequest`
/// * `app_state` - Application state
///
/// # Returns
///
/// Returns 200 OK if signup is successful. Returns 409 Conflict if the user already exists.
pub async fn signup(
    req: web::Json<SignupRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    if let Err(_validation_errors) = req.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let auth_service = &app_state.auth_service;

    match auth_service.signup(&req).await {
        Ok(()) => {
            success_log!("[auth_controller] - [signup] - [message: Successfully signed up]");
            HttpResponse::Ok().finish()
        }
        Err(AuthError::DatabaseError(ref error)) => {
            if let Some(db_error) = error.as_db_error() {
                if db_error.code() == &SqlState::UNIQUE_VIOLATION {
                    error_log!(
                        "[auth_controller] - [signup] - [message: db_error = {}]",
                        db_error
                    );
                    // return HttpResponse::new(StatusCode::CONFLICT);
                    return HttpResponse::Conflict().finish();
                }
            }

            error_log!(
                "[auth_controller] - [signup] - [message: error = {}]",
                error
            );
            HttpResponse::InternalServerError().finish()
        }
        Err(auth_error) => {
            error_log!(
                "[auth_controller] - [signup] - [message: auth_error = {}]",
                auth_error
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

// pub async fn verify_email(
//     req: web::Query<VerificationRequest>,
//     pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
// ) -> impl Responder {
//     let conn = pool.get().await.unwrap();

//     // トークンを使ってユーザーを確認
//     match conn.query(
//         "SELECT user_id FROM email_verification_tokens WHERE token = $1",
//         &[&req.token]
//     ).await {
//         Ok(row) => {
//             let user_id: i32 = row.get(0);

//             // ユーザーを確認済みとしてマーク
//             conn.execute(
//                 "UPDATE users SET email_verified = TRUE WHERE id = $1",
//                 &[&user_id]
//             ).await.unwrap();

//             // 有効の場合、トークンを生成
//             match jwt::create_token(&req.name, user_id) {
//                 Ok(token) => {
//                     // ユーザー情報を作成
//                     let user_data = User {
//                         id: user_id,
//                         name: req.name.clone(),
//                         token,
//                     };
//                     return HttpResponse::Ok().json(user_data);
//                 },
//                 Err(err) => {
//                     logger::log(logger::Header::ERROR, &err.to_string());
//                     return HttpResponse::InternalServerError().finish();
//                 },
//             }
//         },
//         Err(err) => {
//             logger::log(logger::Header::ERROR, &err.to_string());
//             return HttpResponse::InternalServerError().finish();
//         }
//     }
// }

/// Handles user login.
///
/// # Arguments
///
/// * `req` - JSON request data of type `LoginRequest`
/// * `app_state` - Application state
///
/// # Returns
///
/// Returns the authenticated user data if successful. Returns 401 if unauthorized.
pub async fn login(req: web::Json<LoginRequest>, app_state: web::Data<AppState>) -> impl Responder {
    if let Err(_validation_errors) = req.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let auth_service = &app_state.auth_service;

    match auth_service.login(&req).await {
        Ok(Some(user_data)) => HttpResponse::Ok().json(user_data),
        Ok(None) => {
            error_log!("[auth_controller] - [login] - [message: USER NOT FOUND]");
            HttpResponse::Unauthorized().finish()
        }
        Err(auth_error) => {
            error_log!(
                "[auth_controller] - [login] - [message: auth_error = {}]",
                auth_error
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Returns the currently authenticated user's data.
///
/// # Arguments
///
/// * `req` - HTTP request
///
/// # Returns
///
/// Returns the authenticated user's data if the request is valid. Returns 401 if unauthorized.
pub async fn current_user(req: HttpRequest) -> impl Responder {
    match jwt::verify(&req) {
        Ok(user_info) => HttpResponse::Ok().json(user_info),
        Err(error) => {
            error_log!(
                "[auth_controller] - [current_user] - [message: error = {}]",
                error
            );
            HttpResponse::Unauthorized().finish()
        }
    }
}
