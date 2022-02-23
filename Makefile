default: clean build test

clean:
	cargo clean

build:
	cargo fmt && cargo schema && cargo clippy -- -D warnings

test: build
	cargo unit-test

integration-test:
	cargo wasm & cargo integration-test