# Conway Game of Life plus WebAssembly and basic HTTP Server

## How to run

First, you have to choose what server do you want to use for hosting the wasm game of life, you can choose between three different implementations using three different libraries: Crossbeam, Async-std or Tokio. Whichever you choose the functionalities are the same.
- `make start_tokio_web`
- `make start_crossbeam_web`
- `make start_async_std_web`

This will compile the wasm file, the web server binary, and bundle everything in the target directory. In the end it will start the server up as shown here:
![](assets/how_to_run.gif)

## Game of life - Rule 30

Game of life variation based on Elliot Waite's [video](https://www.youtube.com/watch?v=IK7nBOLYzdE), but implemented using Rust and Macroquad.

![](rule30/game_of_life.gif)

It consists in simulating a 1D game of life using the [rule30](https://mathworld.wolfram.com/Rule30.html) (which runs on the lower half of the screen) and using the generated rows as "input" for a 2D game of life (which runs on the upper half of the screen).

The 1D conways only uses black and white colors, and the 2D one uses a gradient of colors to show the amount of time a cell has been alive.
