use actix_web::{get, HttpResponse};

/// Single Sign-On (SSO) endpoint handler
///
/// This endpoint is used for identity management and authentication.
/// It serves as the entry point for SSO authentication flow.
///
/// # Returns
///
/// Returns an HTTP response with:
/// * Status 200 (OK) when the endpoint is accessible
///
/// # Security
///
/// This endpoint should be protected by appropriate security measures
/// and should only be accessible through secure channels.
#[get("/sso")]
pub async fn single_sign_on() -> HttpResponse {
    HttpResponse::Ok().finish()
} 