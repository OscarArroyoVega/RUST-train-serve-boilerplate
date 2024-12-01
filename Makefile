
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

docker-shell-cache: docker-build
	docker run -it \
		-v ${PWD}:/workspace \
		-v cargo-cache:/usr/local/cargo/registry \
		-v target-cache:/workspace/house-price-predictor/target \

docker-terminal:
	@powershell -NoProfile -Command "\
		$$container_id = docker ps --filter 'ancestor=house-price-predictor' --format '{{.ID}}'; \
		if ($$container_id) { \
			Write-Host \"Attaching to container $$container_id...\"; \
			docker exec -it $$container_id /bin/bash \
		} else { \
			Write-Host \"Container not found. Make sure it is running.\" \
		}"



# DEV RUNNABLES FOR TRAINING AND API BINARIES_____________________________________
run-train-dev:
	cargo run --bin train

run-api-dev:
	cargo run --bin api



# CLIENT REQUESTS________________________________________________________________
request-health:
	curl http://localhost:8080/health

request-predict-dev:
	curl -X POST http://localhost:8080/predict \
		-H "Content-Type: application/json" \
		-d '{ \
			"crim": 0.00632, \
			"zn": 18.0, \
			"indus": 2.31, \
			"chas": 0, \
			"nox": 0.538, \
			"rm": 6.575, \
			"age": 65.2, \
			"dis": 4.09, \
			"rad": 1, \
			"tax": 296, \
			"ptratio": 15.3, \
			"b": 396.9, \
			"lstat": 4.98, \
			"medv": 24.0 \
		}' 

