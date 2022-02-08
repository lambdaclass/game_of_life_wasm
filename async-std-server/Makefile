bench-local: start bench stop
	
bench:
	cargo run --bin benchmark 127.0.0.1 8080

start: server.PID

server.PID:
	cargo run --bin async-std-http-sv & echo $$! > $@

stop: server.PID
	kill `cat $<` && rm $<
