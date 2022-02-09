async-start: async-server.PID

async-server.PID:
	cargo run --features="build-async-std" --bin async-std-server & echo $$! > $@
	
async-stop: async-server.PID
	kill `cat $<` && rm $<

async-clean:
	rm async-server.PID

crossbeam-start:
	cargo run --features="build-crossbeam" --bin crossbeam-http-server

crossbeam-build:
	cargo build --features="build-crossbeam" --bin crossbeam-http-server

tokio-server:
	cargo run --features build-tokio --bin tokio-http-server
