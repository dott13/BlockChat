use actix_web::guard::{self, GuardContext};
use jsonwebtoken::{decode, DecodingKey, Validation};
use actix_web::{HttpRequest, HttpResponse, Error, dev::Payload};
use futures::future::LocalBoxFuture;
use futures::FutureExt;

use crate::models::token_model::Claims;

/// A function that takes a list of allowed roles and a handler.
/// It returns a new handler that first checks that the JWT from the
/// Authorization header has a role that is in allowed_roles.
/// If so, it calls the inner handler; otherwise, it returns Unauthorized.
pub fn with_roles<F, Fut>(
    allowed_roles: Vec<&'static str>,
    handler: F,
) -> impl Fn(HttpRequest, Payload) -> LocalBoxFuture<'static, Result<HttpResponse, Error>>
where 
    F: Fn(HttpRequest, Payload) -> Fut + Clone + 'static,
    Fut: std::future::Future<Output = Result<HttpResponse, Error>> + 'static,
{
    move |req: HttpRequest, payload: Payload| {
        let allowed_roles = allowed_roles.clone();
        let handler = handler.clone();
        async move {
            //Extract token from Authorization Header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = auth_str.trim_start_matches("Bearer ").trim();
                        //We set JWT SECRET here
                        if let Ok(secret) = std::env::var("JWT_SECRET") {
                            let decoded_key = DecodingKey::from_secret(secret.as_bytes());
                            if let Ok(token_data) = decode::<Claims>(token, &decoded_key, &Validation::default()) {
                                if allowed_roles.contains(&token_data.claims.role.as_str()) {
                                    return  handler(req, payload).await;
                                }
                            }
                        }
                    }
                }
            }
            Ok(HttpResponse::Unauthorized().json("Unauthorized"))
        }
        .boxed_local()
    }
}