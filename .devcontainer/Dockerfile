FROM rust:1.82-bullseye as builder 

# Install nightly toolchain
RUN rustup install nightly

# Set the working directory
WORKDIR /workspace

# Install required dependencies for building
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    libclang-dev \
    cmake \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set LIBCLANG_PATH
ENV LIBCLANG_PATH=/usr/lib/llvm-11/lib

# Install XGBoost from source
RUN git clone --recursive https://github.com/dmlc/xgboost /tmp/xgboost && \
    cd /tmp/xgboost && \
    mkdir build && \
    cd build && \
    cmake .. && \
    make -j$(nproc) && \
    make install && \
    cd / && \
    rm -rf /tmp/xgboost

# Set library path to include XGBoost
ENV LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

CMD ["bash"]
