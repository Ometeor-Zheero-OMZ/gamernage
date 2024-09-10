use actix_web::{
    HttpResponse,
    Responder, HttpRequest, web, http::StatusCode
};
use postgres::error::SqlState;
use tokio_postgres::NoTls;
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString
    },
    Argon2
};
use crate::{
    api::jwt::jwt, constants::custom_type::AuthRepositoryArc, db::models::{
        auth::{
            LoginRequest,
            SignupRequest
        },
        user::User
    }, libraries::{
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
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let auth_repository: &AuthRepositoryArc = &app_state.auth_repository;

    let conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    match auth_repository.guest_login(&req, &conn).await {
        Ok(Some(user_data)) => {
            HttpResponse::Ok().json(user_data)
        }
        Ok(None) => HttpResponse::Unauthorized().finish(),
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
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // database connection pool
    let conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    // argon2 password hashing logic
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = match argon2.hash_password(&req.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    // insert hashed password as user info into the table `users`
    match conn.execute(
        "INSERT INTO users (name, password, email) VALUES ($1, $2, $3)",
        &[&req.name, &hashed_password, &req.email]
    ).await {
        Ok(_) => {
            logger::log(logger::Header::SUCCESS, "Successfully signed up");
            return HttpResponse::Ok().finish();
        },
        Err(ref err) if err.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
            logger::log(logger::Header::ERROR, "name already exists");
            return HttpResponse::new(StatusCode::CONFLICT);
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
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
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    let conn = match pool.get().await {
        Ok(conn) => conn,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let rows = conn.query(
        "SELECT id, name, password FROM users WHERE name = $1;",
        &[&req.name]
    ).await.unwrap();

    if rows.is_empty() {
        return HttpResponse::Unauthorized().finish();
    }

    let password: String = rows.get(0).unwrap().get("password");
    let id: String = rows.get(0).unwrap().get("id");

    let argon2 = Argon2::default();
    let parsed_hash = match PasswordHash::new(&password) {
        Ok(hash) => hash,
        Err(_) => {
            logger::log(logger::Header::ERROR, "Failed hashing a password");
            return HttpResponse::InternalServerError().finish();
        }
    };

    // パスワードが有効か判定
    match argon2.verify_password(&req.password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            // 有効の場合、トークンを生成
            match jwt::create_token(&req.name, &id) {
                Ok(token) => {
                    // ユーザー情報を作成
                    let user_data = User {
                        id,
                        name: req.name.clone(),
                        token,
                    };
                    return HttpResponse::Ok().json(user_data);
                },
                Err(err) => {
                    logger::log(logger::Header::ERROR, &err.to_string());
                    return HttpResponse::InternalServerError().finish();
                },
            }
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::new(StatusCode::UNAUTHORIZED);
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