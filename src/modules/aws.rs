use std::path::PathBuf;
use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use aws_config::BehaviorVersion;
use anyhow::Result;


/// Create an AWS S3 client to interact with the S3 bucket
async fn create_s3_client() -> Result<aws_sdk_s3::Client, anyhow::Error> {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    
    let s3_client = aws_sdk_s3::Client::new(&config);

    Ok(s3_client)
}

/// Push the model to the AWS s3 bucket (model registry)
pub async fn push_model_to_s3(model_path: &str, bucket_name: &str, key: &str) -> anyhow::Result<()> {
    
    // Create an AWS S3 client to interact with the S3 bucket
    let s3_client = create_s3_client().await?;

    // load model file into memory
    let model_bytes = std::fs::read(model_path)?;

    // Upload the file to S3
    let result = s3_client
        .put_object()
        .bucket(bucket_name)   
        .key(key)
        .body(model_bytes
        .into())
        .send()
        .await?;

    Ok(())
}

/// Download the model from the AWS s3 bucket (model registry)
/// TODO extract this block into a helper function as we use it many times
pub async fn download_model_from_s3(bucket_name: &str, key: &str) -> Result<PathBuf>{

    // Create an AWS S3 client to interact with the S3 bucket
    let s3_client = create_s3_client().await?;

    let download_path = PathBuf::from("DownloadedBostonHousingModel.bin");

    // Download the content of the model from S3
    let response = s3_client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await?;

    // Fetch the file data from the result
    let data = response.body.collect().await?.into_bytes();
    
    // Save the file to disk
    std::fs::write(&download_path, data)?;

    Ok(download_path)
}