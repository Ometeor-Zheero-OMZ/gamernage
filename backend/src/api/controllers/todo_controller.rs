use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::StatusCode;
use crate::{
    api::jwt::jwt,
    libraries::{app_state::AppState, logger}
};
use crate::db::models::todo::{
    RequestCreateTodoItem,
    RequestUpdateTodoItem,
    RequestDeleteTodoItem,
    RequestCompleteTodoItem
};

/// todo データを取得
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// ユーザー情報に紐づく todo データを取得し返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
pub async fn get_todos(
    req: HttpRequest,
    app_state: web::Data<AppState>
) -> impl Responder {
    let todo_service =&app_state.todo_service;
    
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => match todo_service.get_todos(user_data).await {
            Ok(todos) => HttpResponse::Ok().json(todos),
            Err(err) => {
                logger::log(logger::Header::ERROR, &err.to_string());
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

/// todo データを作成
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター：　認証済みユーザー情報
/// * `todo_req` - 作成する todo データ
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// ユーザー情報に紐づく todo データを作成し、RETURNING値を返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
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
            Err(err) => {
                logger::log(logger::Header::ERROR, &err.to_string());
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

/// todo データを更新
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター：　認証済みユーザー情報
/// * `todo_req` - 更新する todo データ
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// 200 ステータスコードを返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
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
            Err(err) => {
                logger::log(logger::Header::ERROR, &err.to_string());
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

/// todo データを論理削除
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター：　認証済みユーザー情報
/// * `todo_req` - 削除する todo データの id
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// 200 ステータスコードを返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
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
            Err(err) => {
                logger::log(logger::Header::ERROR, &err.to_string());
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

/// todo ステータスを更新
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター：　認証済みユーザー情報
/// * `todo_req` - 更新する todo データの user_id
/// * `pool` - DBプール
/// 
/// # 戻り値
/// 
/// 200 ステータスコードを返却
/// データ取得に失敗した場合は、500 ステータスコードを返却
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
            Err(err) => {
                logger::log(logger::Header::ERROR, &err.to_string());
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}