//! # Actix Web Application
//! This is an Actix Web-based application that sets up a server with CORS enabled and JWT authentication middleware. 
//! It connects to a PostgreSQL database and initializes application state.
//!
//! ## Components:
//! - **CORS Configuration**: Allows cross-origin requests from any domain and restricts HTTP methods and headers.
//! - **JWT Middleware**: Handles JWT-based authentication for API routes.
//! - **Database Connection Pool**: Establishes a pool of connections to the PostgreSQL database using `bb8`.
//! - **AppState**: Maintains shared state across different parts of the application, such as the services.
//! - **HttpServer Configuration**: Configures the server to handle 20 concurrent connections and binds to `0.0.0.0:8080`.

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::web::JsonConfig;
use api::middlewares::session_middleware::MemorySession;
use api::middlewares::webauthn_middleware::startup;
use dotenvy::dotenv;

use api::middlewares::jwt_middleware;
use libraries::app_state::AppState;

mod api;
mod constants;
mod db;
mod errors;
mod libraries;
mod tests;

const PROJECT_PATH: &'static str = env!("CARGO_MANIFEST_DIR");

/// Entry point for the Actix Web application.
/// 
/// The main function is responsible for initializing the logger, loading environment variables,
/// establishing a connection pool to the database, setting up the application state, and starting the HTTP server.
///
/// ## Returns
/// - `std::io::Result<()>`: Result indicating the success or failure of the server startup.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenv().ok();

    let pool = db::pool::get_db_pool().await;

    let app_state = AppState::init(&pool);

    let key = Key::generate();

    let (webauthn, webauthn_users) = startup();

    HttpServer::new(move || {
        // Configure CORS to allow any origin and restrict HTTP methods and headers
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "PUT", "POST", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .max_age(60 * 60 * 24); // Cache preflight request for 24 hours

        let session = SessionMiddleware::builder(MemorySession, key.clone())
            .cookie_name("webauthnrs".to_string())
            .cookie_http_only(true)
            .cookie_secure(false)
            .build();

        // Create Actix Web app instance
        App::new()
            .wrap(jwt_middleware::JwtMiddleware)
            .wrap(cors)
            .wrap(session)
            .app_data(JsonConfig::default().limit(4096))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(app_state.clone()))
            .app_data(webauthn.clone())
            .app_data(webauthn_users.clone())
            .service(api::handler::handlers::api_scope())
    })
    .bind("0.0.0.0:8080")?
    .workers(10) // TODO: システムのCPUコア数に基づいて設定したいため、模索中
    .run()
    .await
}