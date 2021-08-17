#[macro_use]
extern crate log;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env::set_var(
        "RUST_LOG",
        "actix_web=debug,actix_server=info,cadence_server=info",
    );
    env_logger::init();

    let bind_addr = env::var("BIND_ADDR").expect("'BIND_ADDR' must be set.");

    info!("Bootstrapping server");

    HttpServer::new(|| App::new().route("/health", web::get().to(health)))
        .bind(&bind_addr)?
        .run()
        .await
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("healthy")
}
