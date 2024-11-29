use actix_web::{HttpServer, App, Responder, HttpResponse, get};
use log::{info, error};
use env_logger;

#[get("/health")]
async fn health() -> impl Responder {
    info!("Health check endpoint called");
    HttpResponse::Ok().body("API is healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting API server...");
    
    HttpServer::new(|| {
        info!("Creating new app instance");
        App::new().service(health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

