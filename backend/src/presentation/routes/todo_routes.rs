use actix_web::web::{delete, get, post, scope};
use actix_web::Scope;
use crate::presentation::handlers::todo_handlers::{get_todos, create_todo, update_todo, delete_todo, complete_todo};

pub fn todo_scope() -> Scope {
    scope("/todo")
        .route("", get().to(get_todos))
        .route("", post().to(create_todo))
        .route("/{id}", post().to(update_todo))
        .route("/{id}", delete().to(delete_todo))
        .route("/change-status/{id}", post().to(complete_todo))
}
