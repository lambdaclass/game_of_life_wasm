async-build:
	cargo clippy --features build-async-std --bin async-std-server
	cargo fmt
	cargo build --features="build-async-std" --bin async-std-server

tokio-build:
	cargo clippy --features build-tokio --bin tokio-http-server
	cargo fmt
	cargo build --features build-tokio --bin tokio-http-server

crossbeam-build:
	cargo clippy --features="build-crossbeam" --bin crossbeam-http-server
	cargo fmt 
	cargo build --features="build-crossbeam" --bin crossbeam-http-server

async-start:
	cargo run --features="build-async-std" --bin async-std-server

crossbeam-start:
	cargo run --features="build-crossbeam" --bin crossbeam-http-server

tokio-start:
	cargo run --features build-tokio --bin tokio-http-server

