# Get the current directory in a cross-platform way (UNIX and Windows)
ifeq ($(OS),Windows_NT)
    PWD := $(subst \,/,$(CURDIR))
else
    PWD := $(shell pwd)
endif

# Use conditional for Windows vs Unix
ifeq ($(OS),Windows_NT)
    CURRENT_DIR := $(shell cd)
else
    CURRENT_DIR := $(shell pwd)
endif


# Load environment variables from .env file_____________________________________________________________________
ifneq (,$(wildcard .env))
    include .env
    ifeq ($(OS),Windows_NT)
        export $(shell powershell -Command "Get-Content .env | Where-Object {$$_ -notmatch '^\s*#' -and $$_.trim() -ne ''} | ForEach-Object {$$_.Split('=')[0]}")
    else
        export $(shell sed -e '/^\#/d' -e '/^$$/d' -e 's/=.*//' .env)
    endif
else
    $(error .env file not found)
endif

# Optional: Get all environment variables from .env file (cross-platform)
ifeq ($(OS),Windows_NT)
    GET_ENV_VARS := powershell -command "Get-Content .env | ForEach-Object { $$_.Split('=')[0] }"
else
    GET_ENV_VARS := grep -v '^#' .env | cut -d= -f1
endif


# DEV CONTAINER RUNNABLES_____________________________________________________________________________________
docker-build:
	docker build -t house-price-predictor -f .devcontainer/Dockerfile .

docker-shell: docker-build
	docker run -it -v ${PWD}:/workspace house-price-predictor

docker-shell-cache-api: docker-build
	docker run -it \
		-v ${PWD}:/workspace \
		-v cargo-cache-api:/usr/local/cargo/registry \
		-v target-cache-api:/workspace/house-price-predictor/target \
		-p 8080:8080 \
		--name house-price-predictor-api \
		house-price-predictor

docker-shell-cache-train: docker-build
	docker run -it \
		-v ${PWD}:/workspace \
		-v cargo-cache-train:/usr/local/cargo/registry \
		-v target-cache-train:/workspace/house-price-predictor/target \
		--name house-price-predictor-train \
		house-price-predictor

docker-terminal-api:
	@powershell -NoProfile -Command "\
		$$container_id = docker ps --filter 'name=house-price-predictor-api' --format '{{.ID}}'; \
		if ($$container_id) { \
			Write-Host \"Attaching to API container $$container_id...\"; \
			docker exec -it $$container_id /bin/bash \
		} else { \
			Write-Host \"API container not found. Make sure it is running.\" \
		}"

docker-terminal-train:
	@powershell -NoProfile -Command "\
		$$container_id = docker ps --filter 'name=house-price-predictor-train' --format '{{.ID}}'; \
		if ($$container_id) { \
			Write-Host \"Attaching to training container $$container_id...\"; \
			docker exec -it $$container_id /bin/bash \
		} else { \
			Write-Host \"Training container not found. Make sure it is running.\" \
		}"


# DEV RUNNABLES FOR TRAINING AND API BINARIES________________________________________________________________
check-env:
	@if [ -z "$(AWS_BUCKET_NAME)" ]; then \
		echo "Error: AWS_BUCKET_NAME is not set"; \
		exit 1; \
	fi
	@if [ -z "$(AWS_KEY)" ]; then \
		echo "Error: AWS_KEY is not set"; \
		exit 1; \
	fi
	@if [ -z "$(AWS_REGION)" ]; then \
		echo "Error: AWS_REGION is not set"; \
		exit 1; \
	fi
	@if [ -z "$(PORT)" ]; then \
		echo "Error: PORT is not set"; \
		exit 1; \
	fi


# Update the run commands to use nightly
run-train-dev: check-env
	cargo +nightly run --bin train -- \
		--bucket-name-s3="$(AWS_BUCKET_NAME)" \
		--key-s3="$(AWS_KEY)" \
		--region="$(AWS_REGION)"

run-train-binary: check-env
	./target/debug/train \
		--bucket-name-s3="$(AWS_BUCKET_NAME)" \
		--key-s3="$(AWS_KEY)" \
		--region="$(AWS_REGION)"

run-api-dev:
	cargo +nightly run --bin api -- \
		--bucket-name-s3="$(AWS_BUCKET_NAME)" \
		--key-s3="$(AWS_KEY)" \
		--region="$(AWS_REGION)" \


kill-api-dev:
	@echo "Checking for API processes..."
	@ps aux | grep "target/debug/api" | grep -v grep || echo "No API process found"
	@echo "Attempting to stop API process..."
	@pkill -f "target/debug/api" || echo "No API process was running"


# CLIENT REQUESTS_____________________________________________________________________________________________
request-health:
	curl http://localhost:8080/health

request-predict-dev:
	curl -X POST http://localhost:8080/predict -H "Content-Type: application/json" -d '{"crim": 0.00632, "zn": 18.0, "indus": 2.31, "chas": 0.0, "nox": 0.538, "rm": 6.575, "age": 65.2, "dis": 4.0900, "rad": 1.0, "tax": 296.0, "ptratio": 15.3, "b": 396.90, "lstat": 4.98}'


# DOCKER FOR PRODUCTION_______________________________________________________________________________________
# Build the production image
docker-build-prod:
	docker build \
		--build-arg BUILDKIT_INLINE_CACHE=1 \
		--cache-from house-price-predictor:prod \
		-t house-price-predictor:prod \
		-f docker/dockerfile .

# Run the training container
docker-run-train-prod:
	docker run -it \
		--env-file .env \
		-e RUN_MODE=train \
		house-price-predictor:prod \
		--bucket-name-s3="$(AWS_BUCKET_NAME)" \
		--key-s3="$(AWS_KEY)" \
		--region="$(AWS_REGION)"

# Run the API container
docker-run-api-prod:
	docker run -d \
		--name house-price-predictor-api \
		--env-file .env \
		-p 8080:8080 \
		house-price-predictor:prod \
		/app/api \
		--bucket-name-s3="$(AWS_BUCKET_NAME)" \
		--key-s3="$(AWS_KEY)" \
		--region="$(AWS_REGION)"

# Stop the API container
docker-stop-api-prod:
	docker stop house-price-predictor-api
	docker rm house-price-predictor-api

# View API logs
docker-logs-api-prod:
	docker logs -f house-price-predictor-api