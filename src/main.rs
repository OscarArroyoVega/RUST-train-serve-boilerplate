// 1. Download the CSV file from the internet
// 2. Load the CSV file into a DataFrame
// 3. Preprocess the data
// 4. Train a XGBoost model
// 5. Push the model to the AWS s3 bucket (model registry)

fn main() -> anyhow::Result<()> {
    println!("Start training script");

    // 1 load data CSV file to disk
    let file_path = download_csv_file()?;

    Ok(())
}

/// use reqwest::blocking::get(url); 
/// block means that the function will wait for the request to complete before returning a value.
/// blocking is used for syncronous code.
/// anyhow is a crate that provides a Result type for error handling.
/// :: is used to call a method on an object.
/// ? is used to return an error.
/// -> is used to specify the return type of the function.
/// <()> is a generic type that represents a tuple with no elements.
/// -> anyhow::Result<()> is a generic type that represents a Result type with no elements.

fn download_csv_file() -> anyhow::Result<(String)> {
        // to declare a variable we use let
        let url: &str = "https://raw.githubusercontent.com/stedy/Machine-Learning-with-R-datasets/master/house_price.csv";
        // return error instead   
        let response = reqwest::blocking::get(url)?;
        // get the csv and save it to in memory data type.
        let bytes = response.bytes()?;
        //copy this bytes to a disk

        let file_path = "boston_housing.csv";   

        // Copy the bytes to a file on disk
        std::fs::write(file_path, bytes)?;  

        Ok((file_path.to_string()))

   }