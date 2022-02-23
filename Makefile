default: build test

build:
	cargo fmt && cargo schema && cargo clippy -- -D warnings

test: build
	cargo unit-test && cargo check --tests && cargo wasm
