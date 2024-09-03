use actix_web::{
    web, HttpRequest, HttpResponse,
    Result, Scope
};
use crate::api::controller::{auth, menu, order, restaurant_table, todo};

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
        .route("/auth/guest_login", web::post().to(auth::guest_login))
        .route("/auth/signup", web::post().to(auth::signup))
        .route("/auth/login", web::post().to(auth::login))
        .route("/auth/current_user", web::get().to(auth::current_user))
        // 確認メール
        // .route("/api/auth/verify_email", web::get().to(auth::verify_email))
        .route("/table", web::get().to(restaurant_table::get_tables))
        .route("/table/{restaurant_table_id}/order", web::get().to(restaurant_table::get_table_orders))
        .route("/table/order", web::delete().to(restaurant_table::delete_orders))
        .route("/order/{order_id}", web::get().to(order::get_order))
        .route("/order", web::post().to(order::add_order))
        .route("/orders", web::post().to(order::add_orders))
        .route("/order", web::delete().to(order::delete_order))
        .route("/order/complete", web::delete().to(order::complete_order))
        .route("/menu", web::get().to(menu::get_menus))
        .route("/menu/{menu_id}", web::get().to(menu::get_menu))
        // TODO一覧取得
        .route("/todos", web::get().to(todo::get_todos))
        // TODO作成
        .route("/todo", web::post().to(todo::create_todo))
        // TODO更新
        .route("/todo", web::post().to(todo::update_todo))
        // TODO削除
        .route("/todo", web::delete().to(todo::delete_todo))
        // TODO完了
        .route("/todo/change-status", web::post().to(todo::complete_todo))
        // 存在しないパスにアクセスしようとした際に handler メソッドをトリガー
        .default_service(web::route().to(handler))
}
