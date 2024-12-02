use polars::prelude::*;
use xgboost::{parameters, Booster, DMatrix};
use xgboost::parameters::learning::Objective;
use xgboost::parameters::tree::TreeBoosterParametersBuilder; 
use xgboost::parameters::BoosterType; 
use std::sync::Mutex;
use parking_lot::RwLock; 

// Change to use RwLock instead of Mutex
pub struct Model01 {
    booster: RwLock<Booster>
}

// Implement Send + Sync explicitly
// This is safe because we're controlling all access to the Booster through RwLock
unsafe impl Send for Model01 {}
unsafe impl Sync for Model01 {}

impl Model01 {
    pub fn new(booster: Booster) -> Self {
        Self {
            booster: RwLock::new(booster)
        }
    }

    pub fn predict(&self, data: &DMatrix) -> anyhow::Result<Vec<f32>> {
        let booster = self.booster.read();
        Ok(booster.predict(data)?)
    }
}

/// Converts Polars DataFrames into XGBoost DMatrix objects
pub fn transform_dataframe_to_dmatrix(
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

// Update the load function to return our new Model01 struct
pub fn load_xgboost_model(model_path: &str) -> anyhow::Result<Model01> {
    let booster = Booster::load(model_path)?;
    Ok(Model01::new(booster))
}