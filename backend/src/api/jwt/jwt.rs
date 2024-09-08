use actix_web::{HttpRequest, http::header::HeaderMap, dev::ServiceRequest};
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

use crate::api::util::message::SVR_MSG;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: String,
    pub sub: String,
    pub exp: usize,
}

/// 基底トレイト
pub trait RequestHeaders {
    fn get_headers(&self) -> &HeaderMap;
}

impl RequestHeaders for HttpRequest {
    fn get_headers(&self) -> &HeaderMap {
        self.headers()
    }
}

impl RequestHeaders for ServiceRequest {
    fn get_headers(&self) -> &HeaderMap {
        self.headers()
    }
}

/// JWTの作成
/// 
/// # 引数
/// 
/// * `name` - ユーザー名
/// * `id` - ユーザーID
/// 
/// # 戻り値
/// 
/// エンコード化したトークンを返却
pub fn create_token(name: &str, id: &str) -> Result<String, jsonwebtoken::errors::Error> {

    // トークンの有効期限 10日
    let days = 60 * 60 * 24 * 10;

    let expiration = SystemTime::now() + Duration::from_secs(days);
    let claims = Claims {
        id: id.to_owned(),
        sub: name.to_owned(),
        exp: expiration.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
}

/// JWTのデコード
/// 
/// # 引数
/// 
/// * `token` - JWT
/// 
/// # 戻り値
/// 
/// デコード化したクレームを返却
pub fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256))
}

pub fn verify <R: RequestHeaders>(req: &R)  -> Result<Claims, String> {
    // リクエストヘッダーから Bearer トークンを抽出できる場合
    if let Some(auth_header) = req.get_headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // 接頭辞の "Bearer" を抽出
            let parts: Vec<&str> = auth_str.split_whitespace().collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                let token = parts[1];
                // トークンを認証し、ユーザー情報をデコード
                match self::decode_token(token) {
                    Ok(user_info) => {
                        return Ok(user_info.claims);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            }
        }
    }
    return Err(SVR_MSG.get("TOKEN_NOT_FOUND_IN_REQUEST_HEADER_MSG").unwrap_or(&"").to_string());
}