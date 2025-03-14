use std::rc::Rc;
use std::task::{Context, Poll};
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::token_model::Claims;

pub struct RoleGuard { 
    allowed_roles: Vec<String>,
}

impl RoleGuard {
    pub fn new(allowed_roles: Vec<&'static str>) -> Self {
        Self {
            allowed_roles: allowed_roles.into_iter().map(String::from).collect(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RoleGuard
where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static, // Ensure B can be boxed
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = RoleGuardMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RoleGuardMiddleware {
            service: Rc::new(service),
            allowed_roles: self.allowed_roles.clone(),
        })
    }
}

pub struct RoleGuardMiddleware<S> {
    service: Rc<S>,
    allowed_roles: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for RoleGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Delegate readiness to the inner service.
    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
         self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let allowed_roles = self.allowed_roles.clone();
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = auth_str.trim_start_matches("Bearer ").trim();
                        if let Ok(secret) = std::env::var("JWT_SECRET") {
                            let decoded_key = DecodingKey::from_secret(secret.as_bytes());
                            if let Ok(token_data) = decode::<Claims>(token, &decoded_key, &Validation::default()) {
                                if allowed_roles.contains(&token_data.claims.role) {
                                    return service.call(req)
                                        .await
                                        .map(|res| res.map_into_boxed_body());
                                }
                            }
                        }
                    }
                }
            }
            let response = HttpResponse::Unauthorized().json("Unauthorized");
            Ok(req.into_response(response))
        })
    }
}
