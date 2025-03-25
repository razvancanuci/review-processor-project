use actix_web::middleware::from_fn;
use actix_web::web;
use crate::handlers::idm_handler::single_sign_on;
use crate::middlewares::auth_middleware::auth_middleware;

pub fn init_idm_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/idm")
            .service(single_sign_on)
            .wrap(from_fn(auth_middleware))
    );
}