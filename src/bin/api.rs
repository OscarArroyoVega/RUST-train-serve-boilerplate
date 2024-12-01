use actix_web::{HttpServer, App, Responder, HttpResponse, get, post, web};
use log::{info, error};
use env_logger;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;  

/// Health check endpoint
#[get("/health")]  // This attribute is used to define the health check endpoint
async fn health() -> impl Responder {
    info!("Health check endpoint called");
    // Ok response
    HttpResponse::Ok().body("API is healthy!")  // Return a 200 OK response with a message
}


#[derive(Deserialize, Debug)]
struct PredictionRequest {
    crim: f64,
    zn: f64,
    indus: f64,
    chas: f64,
    nox: f64,
    rm: f64,
    age: f64,
    dis: f64,
    rad: f64,
    tax: f64,
    ptratio: f64,
    b: f64,
    lstat: f64
}

#[derive(Debug, Serialize)]
struct PredictionResponse {
    prediction: f64
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
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Starting API server...");

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