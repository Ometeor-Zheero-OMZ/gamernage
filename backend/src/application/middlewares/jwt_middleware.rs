//! # JWT ミドルウェア
//! 
//! HTTP リクエストに含まれる JWT トークンを検証
//! 無効または欠如している場合は、`Unauthorized` を返す

use actix_web::{body::EitherBody, dev};
use actix_service::Service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use crate::application::jwt::jwt;

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

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // 認証なしでコールが可能な API パスのリスト
        let exempt_paths = vec![
            "/api/auth/guest_login",
            "/api/auth/signup",
            "/api/auth/login",
            "/api/auth/current_user"
        ];

        let is_exempt = exempt_paths.contains(&request.path());

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
