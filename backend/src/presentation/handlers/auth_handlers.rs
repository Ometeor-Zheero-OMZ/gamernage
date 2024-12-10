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

use actix_web::{HttpResponse, Responder, HttpRequest, web};
use postgres::error::SqlState;
use validator::Validate;
use crate::application::jwt::jwt;
use crate::application::errors::auth_error::AuthError;
use crate::application::states::app_state::AppState;
use crate::domain::entities::auth::{LoginRequest, SignupRequest};
use crate::{app_log, error_log, success_log};

/// ゲストログイン
/// 
/// # 引数
/// 
/// * `req`       - `LoginRequest` 型の HTTP リクエストデータ
/// * `app_state` - アプリケーションの状態を管理する
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(HttpResponse::Ok())`         - ログインが成功した場合、ユーザー情報を返します。
/// - `Err(AuthError::DatabaseError)`  - データベースエラーが発生した場合。
/// - `Err(AuthError::InvalidRequest)` - リクエストの検証エラーが発生した場合。
pub async fn guest_login(
    req: web::Json<LoginRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    if let Err(_validation_errors) = req.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let auth_service = &app_state.auth_service;

    match auth_service.guest_login(&req).await {
        Ok(Some(user_data)) => HttpResponse::Ok().json(user_data),
        Ok(None) => {
            HttpResponse::Unauthorized().finish()
        }
        Err(_auth_error) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// 新規登録
/// 
/// ユーザーが新規登録を行うための処理を行います。
/// 
/// # 引数
/// 
/// * `req`       - `SignupRequest` 型の HTTP リクエストデータ
/// * `app_state` - アプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(())`                - 新規登録が成功した場合、HTTPステータスコード200 (OK) が返されます。
/// - `Conflict()`            - 登録しようとしたユーザーがすでに存在する場合（ユニーク制約違反）、HTTPステータスコード409 (Conflict)が返されます。
/// - `InternalServerError()` - 予期しないエラーが発生した場合、HTTPステータスコード500 (Internal Server Error) が返されます。
/// - `BadRequest()`          - リクエストのバリデーションに失敗した場合、HTTPステータスコード400 (Bad Request)が返されます。
pub async fn signup(
    req: web::Json<SignupRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    if let Err(_validation_errors) = req.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let auth_service = &app_state.auth_service;

    match auth_service.signup(&req).await {
        Ok(()) => {
            success_log!("[auth_controller] - [signup] message: Successfully signed up");
            HttpResponse::Ok().finish()
        }
        Err(AuthError::DatabaseError(ref error)) => {
            if let Some(db_error) = error.as_db_error() {
                if db_error.code() == &SqlState::UNIQUE_VIOLATION {
                    error_log!("[auth_controller] - [signup] - message: db_error = {}", db_error);
                    return HttpResponse::Conflict().finish();
                }
            }

            error_log!("[auth_controller] - [signup] message: error = {}", error);
            HttpResponse::InternalServerError().finish()
        }
        Err(auth_error) => {
            error_log!("[auth_controller] - [signup] message: auth_error = {}", auth_error);
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

/// ログイン
/// 
/// # 引数
/// 
/// * `req`       - `LoginRequest` 型の HTTP リクエストデータ
/// * `app_state` - アプリケーションの状態
/// 
/// # 戻り値
/// 
/// `HttpResponse` 型を返します: 
/// 
/// - `Ok(user_data)`         - ログインに成功した場合、ユーザーのデータを含んだレスポンスを返します。
/// - `Unauthorized()`        - ユーザーが存在しない場合、HTTPステータスコード401 (Unauthorized)を返します。
/// - `InternalServerError()` - 予期しないエラーが発生した場合、500 (InternalServerError)を返します。
pub async fn login(
    req: web::Json<LoginRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    if let Err(_validation_errors) = req.validate() {
        return HttpResponse::BadRequest().finish();
    }

    let auth_service = &app_state.auth_service;

    match auth_service.login(&req).await {
        Ok(Some(user_data)) => HttpResponse::Ok().json(user_data),
        Ok(None) => {
            error_log!("[auth_controller] - [login] message: USER NOT FOUND");
            HttpResponse::Unauthorized().finish()
        }
        Err(auth_error) => {
            error_log!("[auth_controller] - [login] message: auth_error = {}", auth_error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// 認証済みユーザーチェック
/// 
/// ヘッダーに含まれるトークンを使って、認証されたユーザー情報を取得します。
/// 
/// # 引数
/// 
/// * `req` - `HttpRequest` 型のリクエストオブジェクト。ヘッダーにトークンを含むリクエスト
/// 
/// # 戻り値
/// 
/// `Result` 型を返します: 
/// 
/// - `Ok(user_info)` - 認証されたユーザー情報が取得できた場合。ユーザー情報をJSON形式で返します。
/// - `Unauthorized()` - トークンが無効または存在しない場合。認証エラーを返します。
pub async fn current_user(req: HttpRequest) -> impl Responder {
    match jwt::verify(&req) {
        Ok(user_info) => HttpResponse::Ok().json(user_info),
        Err(error) => {
            error_log!("[auth_controller] - [current_user] - [message: error = {}]", error);
            HttpResponse::Unauthorized().finish()
        }
    }
}