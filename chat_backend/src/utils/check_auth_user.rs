use actix_web::{http::header::HeaderMap, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::token_model::Claims;

pub struct AuthenticatedUser(pub Claims);

impl AuthenticatedUser {
    /// Extracts the token claims from the request headers.
    pub fn from_headers(req: &HttpRequest) -> Result<Self, Error> {
        Self::from_headers_ref(req.headers())
    }
    
    /// Extracts the token claims from a header reference.
    pub fn from_headers_ref(headers: &HeaderMap) -> Result<Self, Error> {
        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ").trim();
                    let secret = std::env::var("JWT_SECRET")
                        .map_err(|_| actix_web::error::ErrorUnauthorized("JWT secret missing"))?;
                    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
                    let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())
                        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;
                    return Ok(AuthenticatedUser(token_data.claims));
                }
            }
        }
        Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    }
}


impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        ready(AuthenticatedUser::from_headers(req))
    }
}