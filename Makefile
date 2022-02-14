build-web: target server wasm
	echo 'Done'

server:
	cd http-server && \
	echo 'Building Server' && \
	make $(BC) && \
	echo 'Moving binary into target' && \
	cp target/debug/$(SF) ../target

wasm:
	cd conways-game-of-life-rust-webassembly && \
	echo 'Building wasm' && \
	make build && \
	echo 'Moving wasm into target' && \
	cp target/wasm32-unknown-unknown/debug/conways-game-of-life-webassembly.wasm ../target && \
	cp index.html ../target

start-web: build-web
	cd target && \
	./$(SF)

target:
	mkdir -p target

clean:
	rm -rf target

build-crossbeam-web: target crossbeam-server wasm
	echo 'Done'

crossbeam-server:
	cd http-server && \
	echo 'Building Server' && \
	pwd && \
	make crossbeam-build && \
	echo 'Moving binary into target' && \
	cp target/debug/crossbeam-http-server ../target

start-crossbeam-web: build-web
	cd target && \
	./crossbeam-http-server
