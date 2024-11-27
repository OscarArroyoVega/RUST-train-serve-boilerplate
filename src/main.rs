// 1. Download the CSV file from the internet
// 2. Load the CSV file into a DataFrame
// 3. Preprocess the data
// 4. Train a XGBoost model
// 5. Push the model to the AWS s3 bucket (model registry)

use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("Start training script");

    // 1 load external CSV file to disk
    let csv_file_path = download_csv_file()?;

    // 2 load the CSV file into memory
    let df = load_csv_file(&csv_file_path)?;

    Ok(())
}


fn download_csv_file() -> anyhow::Result<(String)> {

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



fn load_csv_file(file_path: &str) -> anyhow::Result<DataFrame> {

    // load the CSV file from disk into Polars DataFrame
    let df: DataFrame = CsvReader::new(std::fs::File::open(file_path)?)
    .finish()?;

    // print the first 5 rows of the DataFrame
    println!("{:?}", df.head(Some(5)));

    Ok(df)
}
