use actix_web::{
    web::{route, scope},
    HttpRequest,
    HttpResponse,
    Result, Scope
};
use super::{auth_routes::auth_scope, todo_routes::todo_scope};

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
