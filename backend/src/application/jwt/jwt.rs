//! # JWT エンコード & デコード
//!
//! ## 関数
//! 
//! - `create_token`: JWTをエンコード
//! - `decode_token`: JWTをデコード
//! - `verify`:       JWTを検証

use actix_web::{HttpRequest, http::header::HeaderMap, dev::ServiceRequest};
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

use crate::application::helpers::message::AUTH_MSG;
use crate::{app_log, error_log};

/// JWT Claims 構造体
///
/// # フィールド
///
/// * `id`  - ユーザーID.
/// * `sub` - サブジェクト（Eメール）.
/// * `exp` - トークンの有効期限 (UNIX タイムスタンプ).
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: i32,
    pub sub: String,
    pub exp: usize,
}

/// ヘッダーを抽出　トレイト
pub trait RequestHeaders {
    fn get_headers(&self) -> &HeaderMap;
}
/// HttpRequest からヘッダーを抽出
impl RequestHeaders for HttpRequest {
    fn get_headers(&self) -> &HeaderMap {
        self.headers()
    }
}
/// ServiceRequest からヘッダーを抽出
impl RequestHeaders for ServiceRequest {
    fn get_headers(&self) -> &HeaderMap {
        self.headers()
    }
}

/// JWTをエンコード
///
/// # 引数
///
/// * `email` - ユーザーのEメール
/// * `id`    - ユーザーID
///
/// # 戻り値
///
/// * `Result<String, jsonwebtoken::errors::Error>` - エンコード化したトークン
pub fn create_token(email: &str, id: &i32) -> Result<String, jsonwebtoken::errors::Error> {

    // トークンの有効期限 10日
    let days = 60 * 60 * 24 * 10;

    let expiration = SystemTime::now() + Duration::from_secs(days);
    let claims = Claims {
        id: id.to_owned(),
        sub: email.to_owned(),
        exp: expiration.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref())
    )
}

/// JWTをデコード
///
/// # 引数
///
/// * `token` - トークン
///
/// # 戻り値
///
/// * `Result<String, jsonwebtoken::errors::Error>` - エンコード化したトークン
pub fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256)
    )
}

/// JWTを検証
///
/// # 引数
///
/// * `req` - リクエスト
///
/// # 戻り値
///
/// * `Result<Claims, String>` - Claims
pub fn verify <R: RequestHeaders>(req: &R)  -> Result<Claims, String> {
    // リクエストヘッダーから Bearer トークンを抽出できる場合
    if let Some(auth_header) = req.get_headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // 接頭辞の "Bearer" を抽出
            let parts: Vec<&str> = auth_str.split_whitespace().collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                let token = parts[1];
                // トークンを認証し、ユーザー情報をデコード
                match decode_token(token) {
                    Ok(user_info) => {
                        return Ok(user_info.claims);
                    },
                    Err(error) => {
                        error_log!("[jwt] - [verify] error = {}", error);
                        return Err(error.to_string());
                    }
                }
            }
        }
    }
    return Err(AUTH_MSG.get("TOKEN_NOT_FOUND_IN_REQUEST_HEADER_MSG").unwrap_or(&"").to_string());
}