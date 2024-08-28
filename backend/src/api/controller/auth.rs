use actix_web::{
    HttpResponse,
    Responder, HttpRequest, web, http::StatusCode
};
use bcrypt::verify;
use tokio_postgres::NoTls;
use serde::{Deserialize, Serialize};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use crate::{
    api::jwt::jwt,
    db::model::user::User,
    library::logger
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    name: String,
    password: String,
}

pub async fn login(
    req: web::Json<LoginRequest>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let conn = pool.get().await.unwrap();

    // Execute a query using the connection from the pool
    let rows = conn.query(
        "SELECT id,name,password FROM users WHERE name = $1;",
        &[&req.name]
    ).await.unwrap();

    if rows.is_empty() {
        return HttpResponse::Unauthorized().finish();
    }

    let password: String = rows.get(0).unwrap().get("password");
    let id: i32 = rows.get(0).unwrap().get("id");

    // パスワードが有効か判定
    match verify(&req.password, &password) {
        Ok(_) => {
            // 有効の場合、トークンを生成
            match jwt::create_token(&req.name, id) {
                Ok(token) => {
                    // ユーザー情報を作成
                    let user_data = User {
                        id,
                        name: req.name.clone(),
                        token,
                    };

                    HttpResponse::Ok().json(user_data)
                },
                Err(err) => {
                    logger::log(logger::Header::ERROR, &err.to_string());
                    HttpResponse::InternalServerError().finish()
                },
            }
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

/// 認証済みのユーザーデータを返却
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメータ
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
