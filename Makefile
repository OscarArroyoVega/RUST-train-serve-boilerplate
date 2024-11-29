PWD := $(shell pwd | tr '\\' '/')

run-train-dev:
	cargo run --bin train

run-api-dev:
	cargo run --bin api
docker-shell:
	docker run -it -v ${PWD}:/workspace house-price-predictor

#docker-shell-cache:
	docker run -it \
		-v ${PWD}:/workspace \
		-v cargo-cache:/usr/local/cargo/registry \
		-v target-cache:/workspace/house-price-predictor/target \
		house-price-predictor bash

# Use conditional for Windows vs Unix
ifeq ($(OS),Windows_NT)
    CURRENT_DIR := $(shell cd)
else
    CURRENT_DIR := $(shell pwd)
endif

request-health:
	curl http://localhost:8080/health

#run-release:
#	cargo run --release
#	./target/release/house-price-predictor