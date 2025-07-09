## Rust Train&Serve- Boilerpalte  (evolved from *Let's Rust!* at RealWorldML.net) - ready for deployment

![Video_preview](https://github.com/user-attachments/assets/4b865f50-c1a3-4466-a8a7-4b467f856e3c)[Watch video preview](https://github.com/user-attachments/assets/7e0ee4cc-6703-4c7b-b84b-b871cff2e5bc)




This is a boilerplate code for applications that download data from a data source (CSV by default), process the data, train a model (in this case, XGBoost), and push the trained model to an AWS S3 bucket. Then, a second microservice creates a server, downloads the model from the S3 bucket, and opens a request API to serve predictions using the model and the payload given by the client. 
The database used for this boilerplate is the Boston Housing Price dataset in CSV format. All of this is written in Rust, using Polars.
The main funtionalities of the repository were done following Pau Labarta Bajo as the instructor for the cohort "learn Rust together".

#### Features
- Data download and processing
- Model training with XGBoost
- Model storage in AWS S3
- Prediction API server in EC2
- Streamlit frontend to test the API

#### Tech Stack
- Rust
- Python
- Polars
- Cargo
- Docker
- AWS s3, EC2
- ngrok (for local tests)
- streamlit


#### Prerequisites
- Make
- Rust
- Rust-Analyzer (recomemnded)
- Cargo
- Docker
- AWS account

#### Getting Started
1. Clone the repository:
   ```sh
   git clone https://github.com/OscarArroyoVega/RUST-boilerplate-train-API.git
   ```
   
### Usage
The data processing service will download the Boston Housing Price dataset, process it, train an XGBoost model, and upload the model to an AWS S3 bucket.
The prediction API server once deployed will download the trained model from the S3 bucket and serve predictions based on the client payload. A streamlit basic frontend has been built to complete the system.


### Deployment in AWS
For deploying the inference service to an EC2 instance (or any other that supports multiple containers) is recommended to compress the local docker image into a .tar file instead of cloning the repository and building it again inside the instance. This is to avoid cloning again the XGBoost package (slow approach).
Access to the API service is configured to be just accessible from the frontend application.
The API service restarts automatically.
The frontend runs also in docker.




