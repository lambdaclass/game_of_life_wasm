async-start: async-server.PID

async-server.PID:
	cargo run --features="build-async-std" --bin async-std-server & echo $$! > $@
	
async-stop: async-server.PID
	kill `cat $<` && rm $<

crossbeam-start: crossbeam-server.PID

crossbeam-server.PID:
	cargo run --features="build-crossbeam" --bin crossbeam-http-server && echo $$! > $@

crossbeam-stop: crossbeam-server.PID
	kill `cat $<` && rm $<
