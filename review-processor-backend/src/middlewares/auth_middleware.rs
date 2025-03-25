use std::env;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{Error, HttpMessage};
use log::warn;
use crate::utils::jwt::verify_jwt;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("No auth header"));
    }

    let auth_header = auth_header.unwrap().to_str();

    if auth_header.is_err() {
        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
    }

    let auth_header = auth_header.unwrap().strip_prefix("Bearer ");

    if auth_header.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
    }
    
    let jwt = auth_header.unwrap();
    let jwt_validation = verify_jwt(jwt).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;
    
    
    let allowed_domain = env::var("ALLOWED_DOMAIN").expect("ALLOWED_DOMAIN must be set");
    if !jwt_validation.ends_with(&allowed_domain) {
        warn!("Invalid domain: {}", jwt_validation);
        return Err(actix_web::error::ErrorUnauthorized(format!("Invalid domain: {}", jwt_validation)));
    }

    req.extensions_mut().insert(jwt_validation);

    next.call(req).await
}
