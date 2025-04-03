use actix_web::middleware::from_fn;
use actix_web::web;
use crate::handlers::review_handler::add_review;
use crate::middlewares::auth_middleware::auth_middleware;

/// Initializes and configures the review-related routes
///
/// This function sets up the review management endpoints under the `/api/reviews` path.
/// All routes in this scope are protected by the authentication middleware.
///
/// # Arguments
///
/// * `cfg` - The service configuration to which the routes will be added
///
/// # Routes
///
/// * `POST /api/reviews` - Endpoint for creating new reviews
pub fn init_review_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/reviews")
            .service(add_review)
            .wrap(from_fn(auth_middleware))
    );
}