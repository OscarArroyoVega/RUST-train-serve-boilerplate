// 1. Download the CSV file from the internet
// 2. Load the CSV file into a DataFrame
// 3. Preprocess the data
// 4. Train a XGBoost model
// 5. Push the model to the AWS s3 bucket (model registry)

use polars::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() -> anyhow::Result<()> {
    println!("Start training script");

    // 1 load external CSV file to disk
    let csv_file_path = download_csv_file()?;

    // 2 load the CSV file into memory
    let df = load_csv_file(&csv_file_path)?;

    // 3 split the data into training and testing sets
    let (train_df, test_df) = train_test_split(&df, 0.2)?;

    Ok(())
}

/// download the CSV file from the internet and save it to disk
fn download_csv_file() -> anyhow::Result<String> {

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
fn load_csv_file(file_path: &str) -> anyhow::Result<DataFrame> {

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
