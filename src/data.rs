use serde::{Deserialize, Serialize};
use polars::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use anyhow::Result;

/// Prediction request payload
#[derive(Deserialize, Debug)]
pub struct PredictionRequest {
    pub crim: f64,
    pub zn: f64,
    pub indus: f64,
    pub chas: f64,
    pub nox: f64,
    pub rm: f64,
    pub age: f64,
    pub dis: f64,
    pub rad: f64,
    pub tax: f64,
    pub ptratio: f64,
    pub b: f64,
    pub lstat: f64
}

/// Prediction response payload
#[derive(Debug, Serialize)]
pub struct PredictionResponse {
    pub prediction: f64
}

/// download the CSV file from the internet and save it to disk
pub fn download_csv_file(
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
pub fn load_csv_file(file_path: &str
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

