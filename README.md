# Rust + Wasm playground

## How to run

First, you have to choose what server do you want to use for hosting the wasm game of life, you can choose between three different implementations using three different libraries: Crossbeam, Async-std or Tokio. Whichever you choose the functionalities are the same.

For using Crossbeam server:<br>
`make start_tokio_web`<br>
For using Tokio server:<br>
`make start_crossbeam_web`<br>
For using Async-std server:<br>
`make start_async_std_web`<br>

This will compile the wasm file, the web server binary, and bundle everything in the target directory. In the end it will start the server up as shown here:
![](img/how_to_run.gif)
