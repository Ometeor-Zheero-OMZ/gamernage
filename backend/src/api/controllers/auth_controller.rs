use actix_web::{
    HttpResponse,
    Responder,
    HttpRequest,
    web, http::StatusCode
};
use postgres::error::SqlState;
use crate::{
    api::jwt::jwt, db::models::auth::{
        LoginRequest,
        SignupRequest
    }, errors::auth_error::AuthError, libraries::{
        app_state::AppState,
        logger
    }
};

/// ログイン可能なユーザーかどうかを判定
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// * `impl Responder` - HTTPレスポンス
/// 
/// トークンを生成し、認証済みのユーザーデータを返却
/// 認証済みでない場合は、401 を返却
pub async fn guest_login(
    req: web::Json<LoginRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let auth_service = &app_state.auth_service;

    match auth_service.guest_login(&req).await {
        Ok(Some(user_data)) => HttpResponse::Ok().json(user_data),
        Ok(None) => {
            logger::log(logger::Header::ERROR, "User not found");
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// サインアップ処理
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// トークンを生成し、認証済みのユーザーデータを返却
pub async fn signup(
    req: web::Json<SignupRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let auth_service = &app_state.auth_service;

    match auth_service.signup(&req).await {
        Ok(()) => {
            logger::log(logger::Header::SUCCESS, "Successfully signed up");
            HttpResponse::Ok().finish()
        }
        Err(AuthError::DatabaseError(ref err)) => {
            if let Some(db_error) = err.as_db_error() {
                if db_error.code() == &SqlState::UNIQUE_VIOLATION {
                    logger::log(logger::Header::ERROR, "name already exists");
                    return HttpResponse::new(StatusCode::CONFLICT);
                }
            }

            logger::log(logger::Header::ERROR, "database error");
            HttpResponse::InternalServerError().finish()
        }
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
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

/// ログイン処理
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// トークンを生成し、認証済みのユーザーデータを返却
/// 認証済みでない場合は、401 を返却
pub async fn login(
    req: web::Json<LoginRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let auth_service = &app_state.auth_service;

    match auth_service.login(&req).await {
        Ok(Some(user_data)) => HttpResponse::Ok().json(user_data),
        Ok(None) => {
            logger::log(logger::Header::ERROR, "User not found");
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// 認証済みのユーザーデータを返却
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター
/// 
/// # 戻り値
/// 
/// 認証済みのユーザーデータを返却
/// 認証済みでない場合は、401 を返却
pub async fn current_user(req: HttpRequest) -> impl Responder {
    match jwt::verify(&req) {
        Ok(user_info) => HttpResponse::Ok().json(user_info),
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}