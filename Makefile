
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



# DEV CONTAINER RUNNABLES_________________________________________________________
docker-build:
	docker build -t house-price-predictor -f .devcontainer/Dockerfile .

docker-shell: docker-build
	docker run -it -v ${PWD}:/workspace house-price-predictor

docker-shell-cache-api: docker-build
	docker run -it \
		-v ${PWD}:/workspace \
		-v cargo-cache-api:/usr/local/cargo/registry \
		-v target-cache-api:/workspace/house-price-predictor/target \
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


# DEV RUNNABLES FOR TRAINING AND API BINARIES_____________________________________
run-train-dev:
	cargo run --bin train

run-api-dev:
	cargo run --bin api

kill-api-dev:
	@echo "Checking for API processes..."
	@ps aux | grep "target/debug/api" | grep -v grep || echo "No API process found"
	@echo "Attempting to stop API process..."
	@pkill -f "target/debug/api" || echo "No API process was running"

# CLIENT REQUESTS________________________________________________________________
request-health:
	curl http://localhost:8080/health

request-predict-dev:
	curl -X POST http://localhost:8080/predict -H "Content-Type: application/json" -d '{"crim": 0.00632, "zn": 18.0, "indus": 2.31, "chas": 0.0, "nox": 0.538, "rm": 6.575, "age": 65.2, "dis": 4.0900, "rad": 1.0, "tax": 296.0, "ptratio": 15.3, "b": 396.90, "lstat": 4.98}'

