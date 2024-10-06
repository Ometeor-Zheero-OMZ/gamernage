use crate::api::jwt::jwt;
use crate::db::models::community::Community;
use crate::libraries::app_state::AppState;
use crate::{app_log, error_log};
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn create_community(
    req: HttpRequest,
    user_req: web::Json<Community>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let community_service = &app_state.community_service;
    let user = jwt::verify(&req);

    match user {
        Ok(user_data) => {
            match community_service
                .create_community(user_data, &user_req)
                .await
            {
                Ok(response) => HttpResponse::Ok().json(response),
                Err(community_error) => {
                    error_log!("[community_controller] - [create_community] - [message: community_error = {}]", community_error);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(error) => {
            error_log!(
                "[community_controller] - [create_community] - [message: error = {}]",
                error
            );
            HttpResponse::Unauthorized().finish()
        }
    }
}
