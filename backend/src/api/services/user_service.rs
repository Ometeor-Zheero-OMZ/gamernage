//! User Service Module

use crate::api::jwt::jwt::Claims;
use tokio_postgres::Error;

use crate::libraries::logger;

/// Retrieves the user ID from the database using the user's email address.
///
/// # Arguments
///
/// * `user` - User information (of type `&Claims`).
/// * `transaction` - Database transaction (of type `&tokio_postgres::Transaction<'_>`).
///
/// # Returns
///
/// * `Ok(i32)` - User ID if the user exists.
/// * `Err(Error)` - An error if the query fails or the user is not found.
///
/// # Example
///
/// ```rust
/// let user_id = get_user_id(&user, &mut tx).await?;
/// ```
pub async fn get_user_id(user: &Claims, transaction: &tokio_postgres::Transaction<'_>) -> Result<i32, Error> {
    // ユーザーの存在チェック
    let row_result = transaction.query_one(
        r#"
            SELECT
                user_id
            FROM
                user_profiles
            WHERE
                email = $1
        "#,
        &[&user.sub]
    ).await;

    match row_result {
        Ok(user_row) => {
            let user_id = user_row.get("user_id");

            Ok(user_id)
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &format!("[user_service] - [get_user_id] err = {}", err));
            Err(err)
        }
    }
}