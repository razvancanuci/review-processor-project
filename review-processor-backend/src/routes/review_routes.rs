use actix_web::middleware::from_fn;
use actix_web::web;
use crate::handlers::review_handler::add_review;
use crate::middlewares::auth_middleware::auth_middleware;

pub fn init_review_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/reviews")
            .service(add_review)
            .wrap(from_fn(auth_middleware))
    );
}