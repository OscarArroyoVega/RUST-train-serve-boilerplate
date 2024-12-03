#![feature(lazy_cell)]
use tokio; // for async runtime
use dotenv::dotenv;
use log::info;
use clap::Parser;

use house_price_predictor::{
    aws::push_model_to_s3,
    data::{download_csv_file, load_csv_file, train_test_split, split_features_and_target},
    model::train_xgboost_model
};


#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long = "bucket-name-s3")]
    bucket_name_s3: String,
    #[arg(short, long = "key-s3")]
    key_s3: String,
    #[arg(short, long = "region")]
    region: String,
}

fn main() -> anyhow::Result<()> {
    // Initialize the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Training the model...");
    dotenv().ok();

    // Parse command line arguments
    let args = Args::parse();
    // Store values in new variables to avoid move issues
    let bucket_name = if args.bucket_name_s3.is_empty() {
        std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME must be set")
    } else {args.bucket_name_s3.clone()       
    };

    let key = if args.key_s3.is_empty() {
        std::env::var("AWS_KEY").expect("AWS_KEY must be set")
    } else {args.key_s3.clone()   
    };

    info!("AWS Configuration:");
    info!("Bucket: {}", bucket_name);
    info!("Key: {}", key);
    info!("Region: {:?}", args.region);

    // 1 load external CSV file to disk
    let csv_file_path = download_csv_file()?;

    // 2 load the CSV file into memory
    let df = load_csv_file(&csv_file_path)?;

    // 3 split the data into training and testing sets
    let (train_df, test_df) = train_test_split(&df, 0.2)?;

    // 4 split the data into features and target
    let (x_train, y_train) = split_features_and_target(&train_df)?;
    let (x_test, y_test) = split_features_and_target(&test_df)?;

    // 5. Train a XGBoost model
    let model_path = train_xgboost_model(&x_train, &y_train, &x_test, &y_test)?;
    println!("Model saved to: {}", model_path);

    // 6. Push the model to the AWS s3 bucket (model registry)
    info!("AWS Configuration:");
    info!("Bucket: {}", args.bucket_name_s3);
    info!("Key: {}", args.key_s3);
    info!("Region: {:?}", args.region);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    
    info!("Attempting to push model to S3...");
    runtime.block_on(push_model_to_s3(
        &model_path,
        &args.bucket_name_s3,
        &args.key_s3    
    ))?;
    info!("Model successfully pushed to S3 bucket");
    Ok(())
}
