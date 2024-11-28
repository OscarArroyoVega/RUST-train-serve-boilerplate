FROM rust:1.82-bullseye as builder

## Set the working directory
WORKDIR /workspace 

# Install required dependencies for building
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev xgboost libclang-dev && \
    rm -rf /var/lib/apt/lists/*

ENV LIBCLANG_PATH=/usr/lib/llvm-11/lib

## Install required packages
#RUN apt-get update && apt-get install -y \
#    pkg-config \
#    libssl-dev \
#    libclang-dev \
#    cmake \
#    build-essential \
#    git

# Install XGBoost from source
#RUN git clone --recursive https://github.com/dmlc/xgboost /tmp/xgboost && \
#    cd /tmp/xgboost && \
#    mkdir build && cd build && \
#    cmake .. && \
#    make -j$(nproc) && \
#    make install



# Copy the project files into the container
COPY . /workspace

# Diagnostic: List files in /workspace to verify
RUN ls -la /workspace

# Build the application
RUN cargo build --release

# Set the command to run your application
CMD ["cargo", "run", "--release"]
