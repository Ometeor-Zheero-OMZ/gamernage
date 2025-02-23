use actix_web::{HttpResponse, Responder};
use serde_json::json;

use crate::{app_log, success_log};

pub async fn healthcheck() -> impl Responder {
    success_log!("API is healthy");
    HttpResponse::Ok().json(json!({"message": "healthy"}))
}