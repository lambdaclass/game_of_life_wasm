build:
	rustup target add wasm32-unknown-unknown
	cargo build --target wasm32-unknown-unknown

build-release:
	rustup target add wasm32-unknown-unknown
	cargo build --release --target wasm32-unknown-unknown

run: build
	basic-http-server .
	