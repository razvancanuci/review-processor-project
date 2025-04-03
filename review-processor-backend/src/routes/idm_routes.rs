use actix_web::middleware::from_fn;
use actix_web::web;
use crate::handlers::idm_handler::single_sign_on;
use crate::middlewares::auth_middleware::auth_middleware;

/// Initializes and configures the Identity Management (IDM) routes
///
/// This function sets up the IDM-related endpoints under the `/api/idm` path.
/// All routes in this scope are protected by the authentication middleware.
///
/// # Arguments
///
/// * `cfg` - The service configuration to which the routes will be added
///
/// # Routes
///
/// * `GET /api/idm/sso` - Single Sign-On endpoint
pub fn init_idm_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/idm")
            .service(single_sign_on)
            .wrap(from_fn(auth_middleware))
    );
}