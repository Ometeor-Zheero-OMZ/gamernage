use actix_web::{
    web, HttpRequest, HttpResponse,
    Result, Scope
};
use crate::api::controllers::{auth_controller, todo_controller};

/// 存在しないパスにアクセスした際に 404 ステータスコードを返却
/// 
/// # 引数
/// 
/// * `req` - リクエストパラメーター
/// 
/// # 戻り値
/// 
/// 404 ステータスコードを返却
async fn handler(req: HttpRequest) -> Result<HttpResponse> {
    let path = req.path();
    Ok(HttpResponse::NotFound().body(format!("APIが見つかりませんでした： '{}'", path)))
}

/// APIパスを管理
/// 
/// # 引数
/// 
/// なし
/// 
/// # 戻り値
/// 
/// 全APIパスをスコープして返却
pub fn api_scope() -> Scope {
    // /auth/... は未認証でもアクセスできるパス
    // 未認証でもアクセスできるパスを設定する場合、jwt_middeware.rs にて設定してください。
    web::scope("/api")
        .route("/auth/guest_login", web::post().to(auth_controller::guest_login))
        .route("/auth/signup", web::post().to(auth_controller::signup))
        .route("/auth/login", web::post().to(auth_controller::login))
        .route("/auth/current_user", web::get().to(auth_controller::current_user))
        // 確認メール
        // .route("/api/auth/verify_email", web::get().to(auth::verify_email))
        // TODO一覧取得
        .route("/todos", web::get().to(todo_controller::get_todos))
        // TODO作成
        .route("/todo", web::post().to(todo_controller::create_todo))
        // TODO更新
        .route("/todo", web::post().to(todo_controller::update_todo))
        // TODO削除
        .route("/todo", web::delete().to(todo_controller::delete_todo))
        // TODO完了
        .route("/todo/change-status", web::post().to(todo_controller::complete_todo))
        // 存在しないパスにアクセスしようとした際に handler メソッドをトリガー
        .default_service(web::route().to(handler))
}
