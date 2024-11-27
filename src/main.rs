// training script

use anyhow::{Ok, Result};

//to declare a function we use fn 
fn main() {
    println!("Start training script");

    // 1 load data CSV file to disk
    download_csv_file();
}
//    use reqwest::blocking::get(url);

fn download_csv_file() -> anyhow::Result<()> {
        // to declare a variable we use let
        let url = "https://raw.githubusercontent.com/stedy/Machine-Learning-with-R-datasets/master/house_price.csv";
        // return error instead   
        let response = reqwest::blocking::get(url)?;
        // get the csv and save it to in memory data type.
        let bytes = response::bytes()?;
        //copy this bytes to a disk

        Ok(())

//        let mut file = File::create("house_price.csv").await?;
//        copy(response.body(), &mut file).await?;
   }

fn sum_two_float_numbers(a: f64, b: f64) -> f64 {
    a + b
}
fn sum_two_integers(a: i32, b: i32) -> i32 {
    a + b
}

fn sum_two_numbers <T: std::ops::Add<Output = T>, U: std::ops::Add<Output = U>> (a: T, b: U) -> T {
    a + b
}


fn test() -> Result<(), anyhow::Error> {
    let sum = (a: 1, b: 2);
    println!("Sum of a and b is {}", sum);
    Ok(())

    let sum = sum_two_float_numbers(a: 1.0, b: 2.0);
    println!("Sum of a and b is {}", sum);

    let sum = sum_two_integers(a: 1, b: 2);
    println!("Sum of a and b is {}", sum);
    Ok(())

}

