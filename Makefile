run-train-dev:
	cargo run --bin train

run-api-dev:
	cargo run --bin api

docker-shell:
	docker run -it -v ${PWD}:/workspace house-price-predictor

docker-shell-cache:
	docker run -it \
		-v ${PWD}:/workspace house-price-predictor \
		-v cargo-cache:/usr/local/cargo/registry \
		-v target-cache:/workspace/house-price-predictor/target \
		house-price-predictor bash

#run-release:
#	cargo run --release
#	./target/release/house-price-predictor