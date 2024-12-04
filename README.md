# Rust Boilerplate Train API

This is a boilerplate code for applications that download data from a data source (CSV by default), process the data, train a model (in this case, XGBoost), and push the trained model to an AWS S3 bucket. Then, a second microservice creates a server, downloads the model from the S3 bucket, and opens a request API to serve predictions using the model and the payload given by the client. 

The database used for this boilerplate is the Boston Housing Price dataset in CSV format. All of this is written in Rust, using Polars and Cargo.

This project was done following Pau Labarta Bajo as the instructor for this cohort to learn Rust together.

## Features

- Data download and processing
- Model training with XGBoost
- Model storage in AWS S3
- Prediction API server

## Technologies Used

- Rust
- Polars
- Cargo
- Docker

## Getting Started

### Prerequisites

- Rust
- Cargo
- Docker
- AWS account with S3 bucket

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/OscarArroyoVega/RUST-boilerplate-train-API.git
   ```

### Usage
The data processing service will download the Boston Housing Price dataset, process it, train an XGBoost model, and upload the model to an AWS S3 bucket.
The prediction API server will download the trained model from the S3 bucket and serve predictions based on the client payload.

#### License
*You can customize further based on your project specifics and instructions.
