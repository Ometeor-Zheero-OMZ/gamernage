use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use dotenvy::dotenv;

use api::middlewares::jwt_middleware;
use libraries::app_state::AppState;
use presentation::routes::routes::api_scopes;

mod api;
mod constants;
mod db;
mod errors;
mod libraries;

mod application;
mod domain;
mod infrastructure;
mod presentation;
mod tests;

const PROJECT_PATH: &'static str = env!("CARGO_MANIFEST_DIR");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenv().ok();

    let pool = db::pool::get_db_pool().await;
    let app_state = AppState::init(&pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "PUT", "POST", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .max_age(60 * 60 * 24);

        App::new()
            .wrap(jwt_middleware::JwtMiddleware)
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(app_state.clone()))
            .service(api_scopes())
    })
    .bind("0.0.0.0:8080")?
    .workers(20)
    .run()
    .await
}