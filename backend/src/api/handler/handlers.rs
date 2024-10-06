//! API Handlers Module

use crate::api::controllers::{auth_controller, community_controller, todo_controller};
use actix_web::{web, HttpRequest, HttpResponse, Result, Scope};

/// Returns a 404 status code for requests to non-existent paths.
///
/// # Arguments
///
/// * `req` - The HTTP request object (`HttpRequest`). It contains information about the request.
///
/// # Returns
///
/// * `Result<HttpResponse>` - Returns an `HttpResponse` with a 404 status code and an error message indicating that the API was not found.
///
/// # Example
///
/// ```rust
/// let response = handler(req).await;
/// assert_eq!(response.status(), StatusCode::NOT_FOUND);
/// ```
async fn handler(req: HttpRequest) -> Result<HttpResponse> {
    let path = req.path();
    Ok(HttpResponse::NotFound().body(format!("APIが見つかりませんでした： '{}'", path)))
}

/// Manages API paths and scopes them.
///
/// This function sets up the API endpoints and returns them as a scope.
///
/// # Arguments
///
/// None
///
/// # Returns
///
/// * `Scope` - Returns a scope containing all API paths. This allows the specified routes to be added to the API.
///
/// # Example
///
/// ```rust
/// let scope = api_scope();
/// ```
pub fn api_scope() -> Scope {
    // /auth/... paths are accessible without authentication
    // For paths accessible without authentication, configure them in jwt_middleware.rs.
    web::scope("/api")
        // Guest login
        .route(
            "/auth/guest_login",
            web::post().to(auth_controller::guest_login),
        )
        // Sign up
        .route("/auth/signup", web::post().to(auth_controller::signup))
        // Login
        .route("/auth/login", web::post().to(auth_controller::login))
        // Get current user information
        .route(
            "/auth/current_user",
            web::get().to(auth_controller::current_user),
        )
        // 確認メール
        // .route("/api/auth/verify_email", web::get().to(auth::verify_email))
        // TODO list retrieval
        .route("/todos", web::get().to(todo_controller::get_todos))
        // Create TODO
        .route("/todo", web::post().to(todo_controller::create_todo))
        // Update TODO
        .route("/todo", web::post().to(todo_controller::update_todo))
        // Delete TODO
        .route("/todo", web::delete().to(todo_controller::delete_todo))
        // Complete TODO
        .route(
            "/todo/change-status",
            web::post().to(todo_controller::complete_todo),
        )
        .route(
            "/create-community",
            web::post().to(community_controller::create_community),
        )
        .route(
            "/community/{id}",
            web::get().to(community_controller::fetch_community_details)
        )

        // Trigger the handler method for non-existent paths
        .default_service(web::route().to(handler))
}
