build-web: target server wasm
	echo 'Done'

server:
	cd http-server && \
	echo 'Building Server' && \
	pwd && \
	make async-build && \
	echo 'Moving binary into target' && \
	cp target/debug/async-std-server ../target

wasm:
	cd conways-game-of-life-rust-webassembly && \
	echo 'Building wasm' && \
	make build && \
	echo 'Moving wasm into target' && \
	cp target/wasm32-unknown-unknown/debug/conways-game-of-life-webassembly.wasm ../target && \
	cp index.html ../target

start-web: build-web
	cd target && \
	./async-std-server


target:
	mkdir -p target

clean:
	rm -rf target
