//! # JWT Middleware Module
//!
//! This module provides middleware for handling JWT (JSON Web Token) authentication in an Actix Web application.
//! The middleware checks if the incoming request requires authentication and validates the JWT token if necessary.
//!
//! ## Overview
//!
//! The `JwtMiddleware` struct is designed to integrate with Actix Web's service system to enforce authentication on
//! specific endpoints. It uses the `jwt::verify` function to verify the presence and validity of a JWT token in the
//! request. If the token is invalid or missing, the middleware responds with an `Unauthorized` HTTP status. The
//! middleware excludes certain endpoints from authentication checks, allowing unauthenticated access to these routes.
//!
//! ## Dependencies
//!
//! This module depends on the following crates:
//! - `actix_web`: Provides core web server functionality, including middleware and service handling.
//! - `actix_service`: Provides traits and utilities for defining and composing Actix services.
//! - `futures`: Provides utilities for working with asynchronous computations, such as `Future` and `Ready`.
//!
//! ## Usage
//!
//! To use the `JwtMiddleware`, you need to integrate it into your Actix Web application's service configuration.
//! The middleware will be applied to incoming requests, checking JWT tokens for routes that require authentication.
//!
//! # Example
//!
//! ```rust
//! use actix_web::{web, App, HttpServer};
//! use crate::api::jwt::JwtMiddleware;
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| {
//!         App::new()
//!             .wrap(JwtMiddleware)
//!             .service(web::resource("/api/protected").to(protected_handler))
//!     })
//!     .bind("0.0.0.0:8080")?
//!     .run()
//!     .await
//! }
//!
//! async fn protected_handler() -> impl actix_web::Responder {
//!     "This is a protected route!"
//! }
//! ```

use actix_web::{body::EitherBody, dev};
use actix_service::Service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, Ready, LocalBoxFuture};

use crate::api::jwt::jwt;
use crate::{app_log, info_log};

/// Middleware for JWT authentication in Actix Web.
///
/// This middleware checks if a request requires authentication by verifying the JWT token. If the token is invalid
/// or missing, it responds with an `Unauthorized` status. Certain routes are exempt from this check, allowing
/// unauthenticated access.
pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    /// Creates a new `JwtMiddlewareService` with the given service.
    ///
    /// # Arguments
    ///
    /// * `service` - The service that will handle the requests after the middleware has processed them.
    ///
    /// # Returns
    ///
    /// Returns a future that resolves to the `JwtMiddlewareService`.
    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService { service })
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);


    /// Processes the incoming request, applying JWT authentication.
    ///
    /// If the request path is not exempt from authentication, it verifies the JWT token. If the token is invalid
    /// or missing, it returns an `Unauthorized` response. Otherwise, it forwards the request to the wrapped service.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming service request.
    ///
    /// # Returns
    ///
    /// A future that resolves to the response from the service or an `Unauthorized` response if authentication fails.
    fn call(&self, request: ServiceRequest) -> Self::Future {
        let path = request.path();
        info_log!("Request path: {}", path);

        // List of endpoints without permission to access
        let exempt_paths = vec![
            "/api/auth/guest_login",
            "/api/auth/signup",
            "/api/auth/login",
            "/api/auth/current_user",
            "/api/register_start/",
            "/api/register_finish",
            "/api/login_start/",
            "/api/login_finish"
        ];

        // let is_exempt = exempt_paths.contains(&request.path());
        // let is_exempt = exempt_paths.iter().any(|path| request.path().starts_with(path));
        let is_exempt = exempt_paths.iter().any(|&exempt_path| path.starts_with(exempt_path));
        info_log!("Is valid path?: {}", is_exempt);

        if !is_exempt {
            let is_logged_in = match jwt::verify(&request) {
                Ok(_user_info) => true,
                Err(_) => false
            };
    
            if !is_logged_in {
                let (request, _pl) = request.into_parts();
    
                let response = HttpResponse::Unauthorized()
                    .finish()
                    .map_into_right_body();
    
                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        }

        let res = self.service.call(request);

        Box::pin(async move {
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
