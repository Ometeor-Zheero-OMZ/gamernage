use actix_web::{HttpRequest, http::header::HeaderMap, dev::ServiceRequest};
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation, TokenData};
use lambda_http::LambdaEvent;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

use crate::api::utils::message::SVR_MSG;
use crate::{app_log, error_log};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: i32,
    pub sub: String,
    pub exp: usize,
}

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

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256)
    )
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
    return Err(SVR_MSG.get("TOKEN_NOT_FOUND_IN_REQUEST_HEADER_MSG").unwrap_or(&"").to_string());
}

pub async fn verify_from_lambda_event(event: LambdaEvent<serde_json::Value>) -> Result<Claims, String> {
    // Payloadからヘッダーを取得
    if let Some(headers) = event.payload.get("headers").and_then(|h| h.as_object()) {
        // Authorizationヘッダーを探す
        if let Some(auth_header) = headers.get("Authorization").and_then(|h| h.as_str()) {
            // "Bearer" トークンの形式を確認
            let parts: Vec<&str> = auth_header.split_whitespace().collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                let token = parts[1];
                // トークンをデコードしてユーザー情報を取得
                match decode_token(token) {
                    Ok(user_info) => {
                        return Ok(user_info.claims);
                    }
                    Err(e) => {
                        error_log!("[jwt] - [verify_from_lambda_event] - error = {}", e);
                        return Err(e.to_string());
                    }
                }
            }
        }
    }

    // Authorizationヘッダーが見つからなかった場合
    Err("Token not found in request header".to_string())
}