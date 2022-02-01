<div align="center">

  <h1><code>Conways's Game of Life - Rust & WebbAssembly</code></h1>

</div>

## Requirements

### The Rust Toolchain

You will need the standard Rust toolchain, including rustup, rustc, and cargo.

[Follow these instructions to install the Rust toolchain.](https://www.rust-lang.org/tools/install)

The Rust and WebAssembly experience is riding the Rust release trains to stable! That means we don't require any experimental feature flags. However, we do require Rust 1.30 or newer.

### `wasm-pack`

`wasm-pack` is your one-stop shop for building, testing, and publishing Rust-generated WebAssembly.

Use `cargo install wasm-pack` to install [wasm-pack](https://github.com/rustwasm/wasm-pack)

```
cargo install wasm-pack
```

### `npm`

npm is a package manager for JavaScript. We will use it to install and run a JavaScript bundler and development server. At the end of the tutorial, we will publish our compiled .wasm to the npm registry.

Follow these instructions to install npm.

If you already have npm installed, make sure it is up to date with this command:

```
npm install npm@latest -g
```

## Build the project

We use wasm-pack to orchestrate the following build steps:

- Ensure that we have Rust 1.30 or newer and the wasm32-unknown-unknown target installed via rustup,
- Compile our Rust sources into a WebAssembly .wasm binary via cargo,
- Use wasm-bindgen to generate the JavaScript API for using our Rust-generated WebAssembly.

To do all of that, run this command inside the project directory:

```
wasm-pack build
```

## Install the dependencies

```
cd www/
npm install
```

## Serving Locally

```
npm run start
```