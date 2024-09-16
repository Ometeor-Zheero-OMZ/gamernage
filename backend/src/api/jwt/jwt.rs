//! # JWT Encoding and Decoding Module
//!
//! This module provides functionality for creating, decoding, and verifying JWT (JSON Web Token) in a Rust application.
//! It uses the `jsonwebtoken` crate to handle the encoding and decoding of JWT tokens. It also defines a trait for
//! extracting request headers, which is used for token validation in HTTP requests.
//!
//! ## Overview
//!
//! The `jwt` module includes functions for creating JWTs with user information, decoding JWTs to extract claims, 
//! and verifying JWTs from HTTP requests. It uses a fixed secret key for signing the tokens and sets an expiration
//! time for the tokens. The module also provides a trait `RequestHeaders` to handle different types of requests,
//! allowing extraction of headers for token verification.
//!
//! ## Dependencies
//!
//! This module depends on the following crates:
//! - `actix_web`: Provides core web server functionality, including HTTP request handling.
//! - `jsonwebtoken`: Provides functionality for encoding and decoding JWT tokens.
//! - `serde`: Provides serialization and deserialization capabilities for Rust data structures.
//!
//! ## Usage
//!
//! To use this module, you need to integrate the functions into your application logic for token handling. The `create_token`
//! function generates a new JWT with the specified user information. The `decode_token` function decodes an existing JWT to
//! extract the claims. The `verify` function checks if a JWT is valid and extracts user information from it.
//!
//! # Example
//!
//! ```rust
//! use crate::api::jwt;
//!
//! // Creating a token
//! let email = "user@example.com";
//! let id = 1;
//! let token = jwt::create_token(email, &id).expect("Failed to create token");
//!
//! // Verifying a token
//! let req = ...; // Assume req is an instance of `HttpRequest` or `ServiceRequest`
//! match jwt::verify(&req) {
//!     Ok(claims) => println!("Token is valid: {:?}", claims),
//!     Err(err) => println!("Token verification failed: {}", err),
//! }
//! ```

use actix_web::{HttpRequest, http::header::HeaderMap, dev::ServiceRequest};
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration};

use crate::api::utils::message::SVR_MSG;
use crate::libraries::logger;

/// Struct representing JWT claims.
///
/// # Fields
///
/// * `id` - The user ID.
/// * `sub` - The subject (user email).
/// * `exp` - The expiration time of the token (UNIX timestamp).
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: i32,
    pub sub: String,
    pub exp: usize,
}

/// Trait for extracting headers from HTTP requests.
///
/// This trait is implemented for both `HttpRequest` and `ServiceRequest` to provide a unified way to access
/// request headers.
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

/// Creates a JWT token for the specified user.
///
/// # Arguments
///
/// * `email` - The user's email address (`&str`).
/// * `id` - The user's ID (`&i32`).
///
/// # Returns
///
/// * `Result<String, jsonwebtoken::errors::Error>` - The encoded JWT token.
///
/// # Example
///
/// ```rust
/// let token = jwt::create_token(&email, &id)?;
/// ```
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

/// Decodes a JWT token to extract the claims.
///
/// # Arguments
///
/// * `token` - The JWT token (`&str`).
///
/// # Returns
///
/// * `Result<TokenData<Claims>, jsonwebtoken::errors::Error>` - The decoded token data containing the claims.
///
/// # Example
///
/// ```rust
/// match jwt::decode_token(token) {
///     Ok(data) => println!("Token claims: {:?}", data.claims),
///     Err(err) => eprintln!("Failed to decode token: {}", err),
/// }
/// ```
pub fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256)
    )
}

/// Verifies a JWT token from an HTTP request.
///
/// # Arguments
///
/// * `req` - The HTTP request (`R: RequestHeaders`).
///
/// # Returns
///
/// * `Result<Claims, String>` - The claims extracted from the token if it is valid; otherwise, an error message.
///
/// # Example
///
/// ```rust
/// let claims = jwt::verify(&req)?;
/// ```
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
                    Err(err) => {
                        logger::log(logger::Header::INFO, &format!("[jwt] - [verify] err = {}", err));
                        return Err(err.to_string());
                    }
                }
            }
        }
    }
    return Err(SVR_MSG.get("TOKEN_NOT_FOUND_IN_REQUEST_HEADER_MSG").unwrap_or(&"").to_string());
}