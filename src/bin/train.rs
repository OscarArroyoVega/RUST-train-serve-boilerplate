use polars::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use xgboost::{parameters, Booster, DMatrix};
use aws_config::Region;
use aws_config::meta::region::RegionProviderChain;
use tokio; // for async runtime
use dotenv::dotenv;

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

/// download the CSV file from the internet and save it to disk
fn download_csv_file(
) -> anyhow::Result<String> {

    // to declare a variable we use let
    let url: &str = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";  
    
    // get the csv and save it to in memory data type.
    let response = reqwest::blocking::get(url)?;

    // get the csv and save it to in memory data type.
    let bytes = response.bytes()?;
    //copy this bytes to a disk

    // create a file path
    let file_path = "boston_housing.csv";   

    // Copy the bytes to a file on disk
    std::fs::write(file_path, bytes)?;  

    Ok(file_path.to_string())
}

/// load the CSV file from disk into Polars DataFrame
fn load_csv_file(file_path: &str
) -> anyhow::Result<DataFrame> {

    // load the CSV file from disk into Polars DataFrame
    let df: DataFrame = CsvReader::new(std::fs::File::open(file_path)?)
    .finish()?;

    // print the first 5 rows of the DataFrame and some basic information about the DataFrame
    println!("{:?}", df.head(Some(5)));
    println!("Number of rows: {}", df.height());
    println!("Number of columns: {}", df.width());

    // plot a distribution histogram of each one of the columns

    Ok(df)
}

/// Randomly splits the data into training and testing sets
pub fn train_test_split( 
    df: &DataFrame, 
    test_size_percentage: f64
) -> anyhow::Result<(DataFrame, DataFrame)> {

    // Generate a vector from 1 to the number of rows in the DataFrame
    let mut indices: Vec<usize> = (0..df.height()).collect();

    // Create a random number generator
    let mut rng = thread_rng();

    // Shuffle the indices in place
    indices.shuffle(&mut rng);

    // Split the indices into training and testing sets
    let split_idx = (df.height() as f64 * (1.0 - test_size_percentage)).ceil() as usize;

    // Create the training and testing sets
    let train_indices = indices[0..split_idx].to_vec();
    let test_indices = indices[split_idx..].to_vec();

    // Convert from Vec<usize> to ChunkedArray<Int32Type>
    // We do this transformation because the DataFrame::take method
    // expects a ChunkedArray<Int32Type> as an argument.
    let train_indices_ca = UInt32Chunked::from_vec(
        "".into(), train_indices.iter().map(|&x| x as u32).collect());
    let test_indices_ca = UInt32Chunked::from_vec(
        "".into(), test_indices.iter().map(|&x| x as u32).collect());

    // Split the df DataFrame into training and testing sets
    // using the DataFrame::take method.
    let train_df = df.take(&train_indices_ca)?;
    let test_df = df.take(&test_indices_ca)?;
    
    println!("Training set size: {}", train_df.height());
    println!("Testing set size: {}", test_df.height());

    Ok((train_df, test_df))
}

/// Splits the given DataFrame into 2 dataframes: one for features and the other for the target
pub fn split_features_and_target(df: &DataFrame
) -> anyhow::Result<(DataFrame, DataFrame)> {

    let feature_names = vec![
        "crim", "zn", "indus", "chas", "nox", "rm", "age", "dis", "rad", "tax",
        "ptratio", "b", "lstat"
    ];
    let target_name = vec!["medv"];

    let features = df.select(feature_names)?;
    let target = df.select(target_name)?;

    Ok((features, target))
}

/// Converts Polars DataFrames into XGBoost DMatrix objects
fn transform_dataframe_to_dmatrix(
    x_train: &DataFrame,
    y_train: &DataFrame,
    x_test: &DataFrame,
    y_test: &DataFrame,
) -> anyhow::Result<(DMatrix, DMatrix)> {
    // Transform Polars DataFrames into 2D arrays
    let x_train_array = x_train.to_ndarray::<Float32Type>(IndexOrder::C)?;
    let y_train_array = y_train.to_ndarray::<Float32Type>(IndexOrder::C)?;
    let x_test_array = x_test.to_ndarray::<Float32Type>(IndexOrder::C)?;
    let y_test_array = y_test.to_ndarray::<Float32Type>(IndexOrder::C)?;

    // Convert arrays to slices
    let x_train_slice = x_train_array.as_slice()
        .ok_or_else(|| anyhow::anyhow!("Failed to convert x_train to slice"))?;
    let y_train_slice = y_train_array.as_slice()
        .ok_or_else(|| anyhow::anyhow!("Failed to convert y_train to slice"))?;
    let x_test_slice = x_test_array.as_slice()
        .ok_or_else(|| anyhow::anyhow!("Failed to convert x_test to slice"))?;
    let y_test_slice = y_test_array.as_slice()
        .ok_or_else(|| anyhow::anyhow!("Failed to convert y_test to slice"))?;

    // Create DMatrix objects
    let mut dmatrix_train = DMatrix::from_dense(x_train_slice, x_train.height())?;
    dmatrix_train.set_labels(y_train_slice)?;

    let mut dmatrix_test = DMatrix::from_dense(x_test_slice, x_test.height())?;
    dmatrix_test.set_labels(y_test_slice)?;

    Ok((dmatrix_train, dmatrix_test))
}

/// Train a XGBoost model
pub fn train_xgboost_model(
    x_train: &DataFrame, 
    y_train: &DataFrame,
    x_test: &DataFrame,
    y_test: &DataFrame
) -> anyhow::Result<String> {

    // Convert DataFrames to DMatrix using helper function
    let (dmatrix_train, dmatrix_test) = transform_dataframe_to_dmatrix(
        x_train,
        y_train,
        x_test,
        y_test
    )?;

    // Create evaluation sets
    let evaluation_sets = &[
        (&dmatrix_train, "train"),
        (&dmatrix_test, "test")
    ];

    use xgboost::parameters::learning::Objective;
    use xgboost::parameters::BoosterParameters;
    use xgboost::parameters::tree::TreeBoosterParametersBuilder; 
    use xgboost::Booster;
    use xgboost::parameters::BoosterType; 

     // Create booster parameters with tree configuration
     let booster_params = parameters::BoosterParametersBuilder::default()
        .booster_type(
            BoosterType::Tree(
                TreeBoosterParametersBuilder::default()
                    .max_depth(6)
                    .eta(0.3)
                    .build()
                    .unwrap()
            )
        )
        .learning_params(
            parameters::learning::LearningTaskParametersBuilder::default()
                .objective(Objective::RegLinear)
                .build()
                .unwrap()
        )
        .build()
        .unwrap();

    // Create training parameters
    let training_params = parameters::TrainingParametersBuilder::default()
        .dtrain(&dmatrix_train)
        .evaluation_sets(Some(evaluation_sets))
        .booster_params(booster_params)
        .boost_rounds(100)
        .build()
        .unwrap();



    // Train the model
    let model = Booster::train(&training_params)?;

    // print the the model performance
    // Evaluate and print final metrics
    let predictions = model.predict(&dmatrix_test)?;
    println!("\nFinal Metrics:");
    println!("Test predictions (first 5): {:?}", &predictions[..5.min(predictions.len())]);

    // Save the model to a file
    let model_path = "BostonHousingModel.bin";
    model.save(model_path)?;
    println!("Model saved to: {}", model_path);

    Ok(model_path.to_string())
}

/// Loads an XGBoost model from a binary file and returns it
pub fn load_xgboost_model(model_path: &str) -> anyhow::Result<Booster> {
    let model = Booster::load(model_path)?;
    Ok(model) 
}


/// Push the model to the AWS s3 bucket (model registry)
pub async fn push_model_to_s3(model_path: &str, bucket_name: &str, key: &str) -> anyhow::Result<()> {
    
    // Create an AWS S3 client to interact with the S3 bucket
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));
    let config = aws_config::from_env()
        .region(region_provider)
        .load()
        .await;

    let s3_client = aws_sdk_s3::Client::new(&config);

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