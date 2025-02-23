use actix_web::{
    HttpRequest,
    HttpResponse,
    Result, Scope
};
use actix_web::web::{delete, get, patch, post, route, scope};
use crate::{app_log, error_log};
use crate::presentation::handlers::auth_handlers::{login_user, logout_user, register_user};
use crate::presentation::handlers::healthcheck_handler::healthcheck;
use crate::presentation::handlers::user_handlers::{get_user, login_status};
use crate::presentation::handlers::task_handlers::get_tasks;

async fn handler(req: HttpRequest) -> Result<HttpResponse> {
    let path = req.path();
    let uri = req.uri();

    error_log!("[api_routes] APIが見つかりませんでした： path = '{}' uri = '{}'", path, uri);
    Ok(HttpResponse::NotFound().body(format!("APIが見つかりませんでした： '{}'", path)))
}

pub fn api_scopes() -> Scope {
    scope("/api")
        .service(version_scope())
        .default_service(route().to(handler))
}

pub fn version_scope() -> Scope {
    scope("/v1")
        .service(auth_scope())
        .service(task_scope())
}

/// 認証API
fn auth_scope() -> Scope {
    scope("/auth")
        // .route("/guest_login", post().to(guest_login))
        // .route("/signup", post().to(signup))
        // .route("/login", post().to(login))
        // .route("/current_user", get().to(current_user))
        // new api down below (認証後にコールを許可するAPIの仕分けが必要)
        .route("/register", post().to(register_user))
        .route("/login", post().to(login_user))
        .route("/logout", get().to(logout_user))
        .route("/user", get().to(get_user))
        // .route("/user", patch().to(update_user))
        // .route("/admin/users/{id}", delete().to(delete_user))
        // .route("/admin/users", get().to(get_all_users))
        .route("/login-status", get().to(login_status))
        // .route("/verify-email", post().to(verify_email))
        // .route("/verify-email/{verificationToken}", post().to(verify_user))
        // .route("/forgot-password", post().to(forgot_password))
        // .route("/reset-password/{resetPasswordToken}", post().to(reset_password))
        // .route("/change-password", patch().to(change_password))
        .route("healthcheck", get().to(healthcheck))
}

/// task api
fn task_scope() -> Scope {
    scope("")
        // .route("/task/create", post().to(create_task))
        .route("/tasks", get().to(get_tasks))
        // .route("/task/{id}", get().to(get_task))
        // .route("/task/{id}", patch().to(update_task))
        // .route("/task/{id}", delete().to(delete_task))
}