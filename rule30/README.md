## Game of life - Rule 30

Game of life variation based on Elliot Waite's [video](https://www.youtube.com/watch?v=IK7nBOLYzdE), but implemented using Rust and Macroquad.

![](game_of_life.gif)

It consists in simulating a 1D game of life using the [rule30](https://mathworld.wolfram.com/Rule30.html) (which runs on the lower half of the screen) and using the generated rows as "input" for a 2D game of life (which runs on the upper half of the screen).

The 1D conways only uses black and white colors, and the 2D one uses a gradient of colors to show the amount of time a cell has been alive.
