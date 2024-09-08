use crate::{api::jwt::jwt::Claims, library::logger};
use tokio_postgres::Error;

/// get_user_id　user_idの取得
/// 
/// # 引数
/// 
/// * `user` - ユーザー情報
/// * `transaction` - トランザクション
/// 
/// # 戻り値
/// 
/// * `Result<String, Error>`
pub async fn get_user_id(user: &Claims, transaction: &tokio_postgres::Transaction<'_>) -> Result<String, Error> {
    // ユーザーの存在チェック
    let user_row_result = match transaction.query_one(
        r#"
        SELECT id
        FROM users
        WHERE name = $1
        "#,
        &[&user.sub]
    ).await {
        Ok(user_row) => {
            logger::log(logger::Header::INFO, &format!("{:?}", user_row));
            let user_id = user_row.get("id");
            Ok(user_id)
        },
        Err(err) => Err(err)
    };

    logger::log(logger::Header::INFO, &format!("{:?}", user_row_result));
    user_row_result
}