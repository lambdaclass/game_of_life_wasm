# conways-game-of-life-webassembly
Conway's Game of Life WebAssembly

# How To Run

Use `cargo run` to test natively. If you want to run it using webassembly use 

```
make build
basic-http-server . # You can use whatever you want to serve static files
```

To install basic-http-server just run `cargo install basic-http-server`

After that, you can just use 

```
make run
```

instead of `make build` to build and start the server in one step.

# Tests

You can run the test suite using

```
make test
```