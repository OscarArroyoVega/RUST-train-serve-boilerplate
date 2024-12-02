use actix_web::{HttpServer, App, Responder, HttpResponse, get, post, web};
use log::info;
use env_logger;
use dotenv::dotenv;
use house_price_predictor::modules::{aws::download_model_from_s3, data::{PredictionRequest, PredictionResponse}};
use std::sync::Arc;
use house_price_predictor::modules::model::{Model01, load_xgboost_model};
use clap:: Parser;


#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long = "bucket-name-s3")]
    bucket_name_s3: String,
    #[arg(short, long = "key-s3")]
    key_s3: String,
    #[arg(short, long = "region")]
    region: String,
}

/// Application state that will be shared across all the workers of the API endpoints
struct AppState {
    model: Arc<Model01>,
}

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
async fn predict(payload: web::Json<PredictionRequest>) -> Result<HttpResponse, actix_web::Error> {
    info!("Received prediction request: {:?}", payload);
    // For now, return a dummy prediction
    let prediction = 25.0;  // dummy value
    println!("Prediction: {} Payload: {:?}", prediction, payload);
    Ok(HttpResponse::Ok().json(PredictionResponse {prediction}))
}

/// Main function to start the API server.
#[actix_web::main]  // This attribute is used to mark the main function for the actix_web server
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting API server...");
    dotenv().ok();

    // Parse command line arguments
    let args = Args::parse();
    // Store values in new variables to avoid move issues
    let bucket_name = &args.bucket_name_s3;
    let key = &args.key_s3;
    let region = &args.region;

    info!("AWS Configuration:");
    info!("Bucket: {}", bucket_name);
    info!("Key: {}", key);
    info!("Region: {:?}", region);

    // Download the model from the AWS s3 bucket (model registry)
    let model_path = download_model_from_s3(&bucket_name, &key).await.unwrap();
    
    println!("Model downloaded to: {}", model_path);

    // Create the server and bind it to the address and port
    HttpServer::new(move|| {
        // Load the model into memory
        let model = load_xgboost_model(&model_path).unwrap();

        // Create the state data structure that will be shared across all the workers of the API endpoints
        let app_state = Arc::new(AppState {
            model: Arc::new(model),  
        });

        info!("Creating new app instance");

        App::new()
            .service(health)  // Add the health check endpoint as a worker
            .service(predict)  // Add the predict endpoint as a worker
            .app_data(web::Data::new(app_state))  // Add the app state to the app data
    })
    .bind(("127.0.0.1", 8080))?
    .run() 
    .await?;

    Ok(())
}