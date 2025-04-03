use std::env;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{Error, HttpMessage};
use log::warn;
use crate::utils::jwt::verify_jwt;

/// Authentication middleware for validating JWT tokens
///
/// This middleware intercepts incoming requests and validates the JWT token
/// in the Authorization header. It ensures that:
/// 1. The request has an Authorization header
/// 2. The token is in the correct format (Bearer token)
/// 3. The JWT token is valid and not expired
/// 4. The email domain in the token matches the allowed domain
///
/// # Arguments
///
/// * `req` - The incoming service request
/// * `next` - The next middleware or handler in the chain
///
/// # Returns
///
/// Returns a `ServiceResponse` if authentication is successful, or an `Error` if:
/// * No Authorization header is present
/// * The token format is invalid
/// * The JWT validation fails
/// * The email domain doesn't match the allowed domain
///
/// # Environment Variables
///
/// * `ALLOWED_DOMAIN` - The domain that email addresses must end with to be considered valid
pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Check for presence of Authorization header
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("No auth header"));
    }

    // Convert header to string and validate format
    let auth_header = auth_header.unwrap().to_str();
    if auth_header.is_err() {
        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
    }

    // Extract token from Bearer format
    let auth_header = auth_header.unwrap().strip_prefix("Bearer ");
    if auth_header.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
    }
    
    // Verify JWT token
    let jwt = auth_header.unwrap();
    let jwt_validation = verify_jwt(jwt).await
        .map_err(|e| actix_web::error::ErrorUnauthorized(e))?;
    
    // Validate email domain
    let allowed_domain = env::var("ALLOWED_DOMAIN").expect("ALLOWED_DOMAIN must be set");
    if !jwt_validation.ends_with(&allowed_domain) {
        warn!("Invalid domain: {}", jwt_validation);
        return Err(actix_web::error::ErrorUnauthorized(format!("Invalid domain: {}", jwt_validation)));
    }

    // Add validated email to request extensions for use in handlers
    req.extensions_mut().insert(jwt_validation);

    // Continue to next middleware/handler
    next.call(req).await
}
