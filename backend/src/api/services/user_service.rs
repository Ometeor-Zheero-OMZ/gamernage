//! User Service Module

use crate::api::jwt::jwt::Claims;
use crate::{app_log, error_log};
use bb8_postgres::bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use postgres::NoTls;
use tokio_postgres::Error;

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
pub async fn get_user_id(
    user: &Claims,
    conn: &mut PooledConnection<'_, PostgresConnectionManager<NoTls>>,
) -> Result<i32, Error> {
    // ユーザーの存在チェック
    let row_result = conn
        .query_one(
            r#"
            SELECT
                user_id
            FROM
                user_profiles
            WHERE
                email = $1
        "#,
            &[&user.sub],
        )
        .await;

    match row_result {
        Ok(user_row) => {
            let user_id = user_row.get("user_id");

            Ok(user_id)
        }
        Err(error) => {
            error_log!(
                "[user_service] - [get_user_id] - [message: error = {}]",
                error
            );
            Err(error)
        }
    }
}
