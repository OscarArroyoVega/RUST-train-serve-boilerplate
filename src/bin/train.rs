use polars::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use xgboost::{parameters, Booster, DMatrix};
use aws_config::Region;
use aws_config::meta::region::RegionProviderChain;
use tokio; // for async runtime
use dotenv::dotenv;

use house_price_predictor::modules::{
    aws::push_model_to_s3,
    data::{download_csv_file, load_csv_file, train_test_split, split_features_and_target},
    model::{train_xgboost_model, transform_dataframe_to_dmatrix}
};

fn main() -> anyhow::Result<()> {
    // load the environment variables from the .env file
    dotenv().ok();

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
    let bucket_name = std::env::var("AWS_BUCKET_NAME")?;
    let key = std::env::var("AWS_KEY")?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    
    runtime.block_on(push_model_to_s3(
        &model_path,
        &bucket_name,
        &key    
     ))?;
    println!("Model pushed to S3 bucket");

    Ok(())
}
