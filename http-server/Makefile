async-start: async-server.PID

async-server.PID:
	cargo run --features="build-async-std" --bin async-std-server & echo $$! > $@
	
async-stop: async-server.PID
	kill `cat $<` && rm $<

clean:
	rm async-server.PID

tokio-server:
	cargo run --features build-tokio --bin tokio-http-server
