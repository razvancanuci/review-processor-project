use actix_web::web;
use crate::routes::idm_routes::init_idm_routes;
use crate::routes::review_routes::init_review_routes;

mod idm_routes;
mod review_routes;


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    init_review_routes(cfg);
    init_idm_routes(cfg);
}
