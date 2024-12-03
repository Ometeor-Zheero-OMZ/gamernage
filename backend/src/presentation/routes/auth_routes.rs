use actix_web::{
    web, HttpRequest, HttpResponse,
    Result, Scope
};
use crate::api::controllers::{auth_controller, todo_controller};

pub fn auth_scope() -> Scope {
    web::scope("/api")
        .route("/auth/guest_login", web::post().to(auth_controller::guest_login))
        .route("/auth/signup", web::post().to(auth_controller::signup))
        .route("/auth/login", web::post().to(auth_controller::login))
        .route("/auth/current_user", web::get().to(auth_controller::current_user))
        // .route("/api/auth/verify_email", web::get().to(auth::verify_email))
}
