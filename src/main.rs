#[macro_use]
extern crate log;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

mod logger;
mod models;

// Convenience type for error trait object
type AppError = Box<dyn std::error::Error>;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // Load environment variables from '.env'
    dotenv::dotenv()?;
    // Initialize custom logger
    logger::setup_logger("LOG_LEVEL", "LOG_FILE")?;

    let bind_addr = env::var("BIND_ADDR").expect("'BIND_ADDR' must be set.");

    info!("Bootstrapping server");

    HttpServer::new(|| App::new().route("/health", web::get().to(health)))
        .bind(&bind_addr)?
        .run()
        .await?;

    Ok(())
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("healthy")
}
