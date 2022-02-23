default: clean build test

clean:
	cargo clean

build:
	cargo fmt && cargo schema && cargo wasm && cargo clippy -- -D warnings

test: build
	cargo unit-test

integration-test: build
	cargo integration-test --no-default-features -- --nocapture