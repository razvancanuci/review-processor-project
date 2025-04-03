//! Review Processor Backend Application
//!
//! This is the main entry point for the Review Processor backend service.
//! It sets up the HTTP server with necessary middleware and configurations.

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

/// Main entry point for the Review Processor backend
///
/// This function:
/// 1. Initializes logging
/// 2. Loads environment variables
/// 3. Configures Sentry for error tracking
/// 4. Sets up CORS for frontend communication
/// 5. Configures the HTTP server with middleware
/// 6. Binds the server to port 8000
///
/// # Environment Variables
///
/// * `SENTRY_DSN` - Sentry DSN for error tracking
/// * `FRONTEND_URL` - URL of the frontend application for CORS
///
/// # Returns
///
/// Returns a `std::io::Result<()>` indicating success or failure
///
/// # Panics
///
/// This function will panic if:
/// * Required environment variables are not set
/// * The server fails to bind to the specified port
#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    // Initialize logging
    env_logger::init();
    
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize Sentry for error tracking
    let _guard = sentry::init((
        env::var("SENTRY_DSN").expect("SENTRY_DSN must be set"),
        sentry::ClientOptions {
            sample_rate: 1.0,
            traces_sample_rate: 0.3,
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
    
    // Configure and start the HTTP server
    HttpServer::new(move || {
        // Configure CORS for frontend communication
        let cors = Cors::default()
            .allowed_origin(
                env::var("FRONTEND_URL")
                    .expect("FRONTEND_URL must be set")
                    .as_str(),
            )
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        // Build the application with middleware and routes
        App::new()
            .wrap(Sentry::new())  // Add Sentry error tracking
            .wrap(cors)           // Add CORS support
            .wrap(Logger::default())  // Add request logging
            .configure(routes::init_routes)  // Configure application routes
    })
    .bind(("0.0.0.0", 8000))?  // Bind to all interfaces on port 8000
    .run()
    .await
}
