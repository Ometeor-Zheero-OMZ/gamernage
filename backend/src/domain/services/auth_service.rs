//! # 認証サービス
//! 
//! 認証処理を定義したサービス
//! 
//! ## メソッド
//! 
//! `guest_login` - ゲストログイン
//! `signup`      - 新規登録
//! `login`       - ログイン

use async_trait::async_trait;
use argon2::Argon2;
use argon2::password_hash::{
    rand_core::OsRng,
    PasswordHash,
    PasswordHasher,
    PasswordVerifier,
    SaltString
};
use crate::{
    application::errors::auth_error::AuthError,
    application::jwt::jwt,
    application::types::di_type::AuthRepositoryArc,
    domain::entities::auth::{LoginRequest, SignupRequest},
    domain::entities::user::User,
    {app_log, error_log}
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError>;
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
}

pub struct AuthServiceImpl {
    auth_repository: AuthRepositoryArc,
}

impl AuthServiceImpl {
    pub fn new(auth_repository: AuthRepositoryArc) -> Self {
        AuthServiceImpl { auth_repository }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    /// ゲストログイン
    /// 
    /// ゲストユーザーとしてログインするための認証を行います。
    /// 
    /// # 引数
    /// 
    /// * `req` - ログイン情報を含む `LoginRequest` 型の JSON データ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(Some(User))` - 認証が成功し、ゲストログインが完了した場合、`User` 型のデータを返します。
    /// - `Ok(None)`       - 認証が失敗した場合（例: ユーザーが存在しない、パスワードが一致しない）。
    /// - `Err(AuthError)` - 認証処理中にエラーが発生した場合、カスタムエラーを返します。
    async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError> {
        let user_opt = self.auth_repository.guest_login(req).await?;
        match user_opt {
            Some(user) => Ok(Some(user)),
            None => Ok(None)
        }
    }
    /// 新規登録
    /// 
    /// 新しいユーザーアカウントを作成します。
    /// 
    /// # 引数
    /// 
    /// * `req` - 新規登録情報を含む `SignupRequest` 型の JSON データ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// - `Ok(())`         - 新規登録が成功した場合、値を返しません。
    /// - `Err(AuthError)` - 新規登録に失敗した場合、カスタムエラーを返します。
    async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError> {
        // パスワードを暗号化
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = argon2.hash_password(&req.password.as_bytes(), &salt)?.to_string();
        
        self.auth_repository.signup(&req.name, &req.email, &hashed_password).await?;

        Ok(())
    }
    /// ログイン
    /// 
    /// ログインするための認証を行います。
    /// 
    /// # 引数
    /// 
    /// * `req` - ユーザーのログイン情報を含む `LoginRequest` 型の JSON データ
    /// 
    /// # 戻り値
    /// 
    /// `Result` を返します:
    /// 
    /// - `Ok(Some(User))` - 認証が成功した場合、ログインに成功したユーザーのデータを含む `User` を返します。
    /// - `Ok(None)`       - 認証に失敗した場合（例: パスワードが一致しない、ユーザーが存在しない）。
    /// - `Err(AuthError)` - 処理中にエラーが発生した場合、カスタムエラーを返します。
    async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError> {
        if let Some((id, name, email, hashed_password)) = self.auth_repository.get_user_by_email(&req.email).await? {
            // ハッシュ化されたパスワードをパース
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&hashed_password)?;

            // 検証
            if let Err(err) = argon2.verify_password(req.password.as_bytes(), &parsed_hash) {
                error_log!("[auth_service] - [login] - [message: Authentication Failed] - Error: {:?}", err);
                return Ok(None);
            }
    
            let token = jwt::create_token(&req.email, &id)?;
            let user_data = User {
                id,
                name,
                email,
                token,
            };
    
            return Ok(Some(user_data));
        }

        Ok(None)
    }
}