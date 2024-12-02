use actix_web::{HttpServer, App, Responder, HttpResponse, get, post, web};
use log::info;
use env_logger;
use serde::{Deserialize, Serialize};
use std::fmt::Debug; 
use dotenv::dotenv;
use house_price_predictor::modules::{aws::download_model_from_s3, data::{PredictionRequest, PredictionResponse}};

/// Health check endpoint
#[get("/health")]  // This attribute is used to define the health check endpoint
async fn health() -> impl Responder {
    info!("Health check endpoint called");
    // Ok response
    HttpResponse::Ok().body("API is healthy!")  // Return a 200 OK response with a message
}

/// Predict endpoint
/// Accepts a JSON payload request with the features and returns a JSON response with the prediction
#[post("/predict")]
async fn predict(payload: web::Json<PredictionRequest>) -> impl Responder {
    info!("Received prediction request: {:?}", payload);
    // For now, return a dummy prediction
    let prediction = 25.0;  // dummy value
    println!("Prediction: {} Payload: {:?}", prediction, payload);
    HttpResponse::Ok().json(PredictionResponse {prediction})
}

/// Main function to start the API server.
#[actix_web::main]  // This attribute is used to mark the main function for the actix_web server
async fn main() -> std::io::Result<()> {
    // Load .env file and handle error if it fails
    if let Err(e) = dotenv() {
        eprintln!("Warning: Failed to load .env file: {}", e);
    }
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Starting API server...");

    // Download the model from the AWS s3 bucket (model registry)
    let bucket_name = std::env::var("AWS_BUCKET_NAME")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let key = std::env::var("AWS_KEY")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let model_path = download_model_from_s3(&bucket_name, &key).await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    println!("Model downloaded to: {}", model_path.display());

    // Create the server and bind it to the address and port
    HttpServer::new(|| {
        info!("Creating new app instance"); 
        App::new()
            .service(health)  // Add the health check endpoint as a worker
            .service(predict)  // Add the predict endpoint as a worker
    })
    .bind(("127.0.0.1", 8080))?
    .run() 
    .await?;

    Ok(())
}