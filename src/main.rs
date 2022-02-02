use macroquad::prelude::*;

#[macroquad::main(window_configuration)]
async fn main() {
    let cell_size = 20.0;
    loop {
        clear_background(RED);
        draw_dead_cells(cell_size);
        draw_grid(cell_size);
        next_frame().await
    }
}

// Windows size config is ignored in wasm/android
fn window_configuration() -> Conf {
    Conf {
        window_height: 600,
        window_width: 800,
        fullscreen: false,
        ..Default::default()
    }
}

fn draw_dead_cells(cell_size: f32) {
    let mut x = 0.0;
    let mut y = 0.0;
    while x + cell_size <= screen_width() {
        while y + cell_size <= screen_height() {
            draw_rectangle(x, y, cell_size, cell_size, WHITE);
            y += cell_size;
        }
        x += cell_size;
        y = 0.0;
    }
}

fn draw_grid(step: f32) {
    let mut x = 0.0;
    let mut y = 0.0;
    while x + step <= screen_width() {
        draw_line(x, 0.0, x, screen_height(), 1.0, LIGHTGRAY);
        x += step;
    }
    while y + step <= screen_height() {
        draw_line(0.0, y, screen_width(), y, 1.0, LIGHTGRAY);
        y += step;
    }
}