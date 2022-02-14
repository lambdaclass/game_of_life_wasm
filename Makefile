define start_web
	cd target && \
	./$(1)
endef

define build_server
	cd http-server && \
	echo 'Building Server' && \
	make $(1) && \
	echo 'Moving binary into target' && \
	cp target/debug/$(2) ../target
endef

wasm:
	cd conways-game-of-life-rust-webassembly && \
	echo 'Building wasm' && \
	make build && \
	echo 'Moving wasm into target' && \
	cp target/wasm32-unknown-unknown/debug/conways-game-of-life-webassembly.wasm ../target && \
	cp index.html ../target

start_tokio_web: target wasm
	$(call build_server,tokio-build,tokio-http-server)
	$(call start_web,tokio-http-server)

start_crossbeam_web: target wasm
	$(call build_server,crossbeam-build,crossbeam-http-server)
	$(call start_web,crossbeam-http-server)

start_async_std_web: target wasm
	$(call build_server,async-build,async-std-server)
	$(call start_web,async-std-server)

target:
	mkdir -p target

clean:
	rm -rf target
