//! # ユーザーサービス
//! 
//! ユーザー処理を定義したサービス
//! 
//! ## メソッド
//! 
//! `get_user_id`     - ユーザーID取得

use async_trait::async_trait;
use crate::{
    app_log, application::{errors::user_error::UserError, jwt::jwt::Claims, types::di_type::UserRepositoryArc}, domain::entities::user::{UserRequest, UserResponse}, error_log
};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_id(&self, user: &Claims) -> Result<i32, UserError>;
    async fn find_user_by_id(&self, req: &UserRequest) -> Result<Option<UserResponse>, UserError>;
}

pub struct UserServiceImpl {
    user_repository: UserRepositoryArc,
}

impl UserServiceImpl {
    pub fn new(user_repository: UserRepositoryArc) -> Self {
        UserServiceImpl { user_repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    /// ユーザーID取得
    /// 
    /// ユーザーIDを取得します。
    /// 
    /// # 引数
    /// 
    /// * `user` - `Claims` 型のユーザーデータ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(i32)`           - ユーザーIDを取得した場合、ユーザーIDを返します。
    /// - `Err(UserError)`    - ユーザーID取得処理中にエラーが発生した場合、カスタムエラーを返します。
    async fn get_user_id(&self, user: &Claims) -> Result<i32, UserError> {
        let user_id_opt= self.user_repository.get_user_id(user).await?;

        match user_id_opt {
            Some(user_id) => {
                Ok(user_id)
            },
            None => {
                error_log!("[user_service] - user not found for email: {}", user.sub);
                Err(UserError::UserNotFound)
            }
        }
    }

    async fn find_user_by_id(&self, req: &UserRequest) -> Result<Option<UserResponse>, UserError> {
        self.user_repository.find_user_by_id(&req.user_id).await
    }
}