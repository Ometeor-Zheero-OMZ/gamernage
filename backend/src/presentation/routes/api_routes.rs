use actix_web::{
    HttpRequest,
    HttpResponse,
    Result, Scope
};
use actix_web::web::{route, scope, get, post, delete};
use crate::presentation::handlers::auth_handlers::{current_user, guest_login, login, signup};
use crate::presentation::handlers::todo_handlers::{get_todos, create_todo, update_todo, delete_todo, complete_todo};

async fn handler(req: HttpRequest) -> Result<HttpResponse> {
    let path = req.path();
    Ok(HttpResponse::NotFound().body(format!("APIが見つかりませんでした： '{}'", path)))
}

pub fn api_scopes() -> Scope {
    scope("/api")
        .service(auth_scope())
        .service(todo_scope())
        .default_service(route().to(handler))
}

/// 認証API
fn auth_scope() -> Scope {
    scope("/auth")
        .route("/guest_login", post().to(guest_login))
        .route("/signup", post().to(signup))
        .route("/login", post().to(login))
        .route("/current_user", get().to(current_user))
        // .route("/verify_email", get().to(auth::verify_email))
}

/// タスクAPI
fn todo_scope() -> Scope {
    scope("/todo")
        .route("", get().to(get_todos))
        .route("", post().to(create_todo))
        .route("/{id}", post().to(update_todo))
        .route("/{id}", delete().to(delete_todo))
        .route("/change-status/{id}", post().to(complete_todo))
}
