build:
	rustup target add wasm32-unknown-unknown
	cargo build --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/debug/conways-game-of-life-webassembly.wasm .

build-release:
	rustup target add wasm32-unknown-unknown
	cargo build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/conways-game-of-life-webassembly.wasm .

run: build
	basic-http-server .

clean:
	rm -rf target 
	rm -rf conways-game-of-life-webassembly.wasm
	
test: 
	cargo test
	
