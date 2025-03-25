mod routes;
mod handlers;
mod middlewares;
mod database;
mod services;
mod models;
mod repositories;
mod utils;

use std::env;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use sentry_actix::Sentry;

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    env_logger::init();
    dotenv().ok();
    
    
    let _guard = sentry::init((
        env::var("SENTRY_DSN").expect("SENTRY_DSN must be set"),
        sentry::ClientOptions {
            sample_rate: 1.0,
            traces_sample_rate: 0.3,
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(
                env::var("FRONTEND_URL")
                    .expect("FRONTEND_URL must be set")
                    .as_str(),
            )
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);
        App::new()
            .wrap(Sentry::new())
            .wrap(cors)
            .wrap(Logger::default())
            .configure(routes::init_routes)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
