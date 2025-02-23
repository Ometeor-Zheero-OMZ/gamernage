use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use crate::application::jwt::jwt;
use crate::application::states::app_state::AppState;
use crate::domain::entities::user::UserRequest;
use crate::{app_log, error_log, success_log};

pub async fn get_user(
    req: HttpRequest,
    user_req: web::Json<UserRequest>,
    app_state: web::Data<AppState>
) -> impl Responder {
    let user = jwt::verify(&req);

    match user {
        Ok(_user_data) => {
            let user_service = &app_state.user_service;

            match user_service.find_user_by_id(&user_req).await {
                Ok(Some(user)) => {
                    HttpResponse::Ok().json(user)
                }
                Ok(None) => {
                    HttpResponse::NotFound().json(json!({ "message": "User not found"}))
                }
                Err(err) => {
                    error_log!("Internal server error: {:?}", err);
                    HttpResponse::InternalServerError().json(json!({ "message": "Internal server error"}))
                }
            }
        },
        Err(_) => HttpResponse::Unauthorized().json(json!({ "message": "Invalid token"}))
    }
}

pub async fn login_status(
    req: HttpRequest,
) -> impl Responder {
    match jwt::verify(&req) {
        Ok(_claims) => {
            success_log!("[user_handler] - [login_status] message: Authorized!");
            HttpResponse::Ok().json(true)
        },
        Err(_) => {
            error_log!("[user_handler] - [login_status] Not authorized");
            HttpResponse::Unauthorized().json(false)
        }
    }
}