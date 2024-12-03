use actix_web::{
    web, HttpRequest, HttpResponse,
    Result, Scope
};
use crate::api::controllers::{auth_controller, todo_controller};

pub fn todo_scope() -> Scope {
    web::scope("/api")
        .route("/todos", web::get().to(todo_controller::get_todos))
        .route("/todo", web::post().to(todo_controller::create_todo))
        .route("/todo", web::post().to(todo_controller::update_todo))
        .route("/todo", web::delete().to(todo_controller::delete_todo))
        .route("/todo/change-status", web::post().to(todo_controller::complete_todo))
}
