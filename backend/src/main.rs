use lambda_http::{service_fn, Error as LambdaError, LambdaEvent};
use serde_json::{json, Value};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use api::lambda_handlers::auth_handler::{
    guest_login,
    signup,
    login,
    current_user
};
use libraries::app_state::AppState;

mod api;
mod constants;
mod db;
mod errors;
mod libraries;
mod tests;

const PROJECT_PATH: &'static str = env!("CARGO_MANIFEST_DIR");

fn cors_response(data: Value) -> Value {
    let mut response = data;
    response["headers"] = json!({
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
        "Access-Control-Allow-Headers": "Content-Type, Authorization",
    });
    response
}

/// AWS Lambda用ハンドラー
async fn function_handler(lambda_event: LambdaEvent<Value>, app_state: Arc<AppState>) -> Result<serde_json::Value, LambdaError> {
    let (event, _context) = lambda_event.clone().into_parts();

    let path = event
        .get("path")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let method = event
        .get("httpMethod")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_uppercase();

    let result = match (method.as_str(), path) {
        ("POST", "/api/auth/guest_login") => guest_login(lambda_event, &app_state).await,
        ("POST", "/api/auth/signup") => signup(lambda_event, &app_state).await,
        ("POST", "/api/auth/login") => login(lambda_event, &app_state).await,
        ("GET", "/api/auth/current_user") => current_user(lambda_event).await,
        _ => Ok(json!({ 
            "error": "Unknown endpoint",
            "message": format!("Method {} not supported for {}", method, path)
        })),
    };

    let response = match result {
        Ok(data) => cors_response(data),
        Err(e) => cors_response(json!({ "error": e.to_string() })),
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenv().ok();

    let pool = db::pool::get_db_pool().await;
    let app_state = Arc::new(AppState::init(&pool));

    lambda_runtime::run(service_fn(|event| {
        function_handler(event, app_state.clone())
    }))
    .await?;
    Ok(())
}
