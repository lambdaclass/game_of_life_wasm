use macroquad::prelude::*;

#[macroquad::main(window_configuration)]
async fn main() {
    let cell_size = 20.0;
    clear_background(RED);
    loop {
        let padding = get_board_padding(cell_size);
        draw_dead_cells(cell_size, padding);
        draw_grid(cell_size, padding);
        next_frame().await
    }
}

fn get_board_padding(cell_size: f32) -> (f32, f32) {
    (
        (screen_width() % cell_size) / 2.0,
        (screen_height() % cell_size) / 2.0,
    )
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

fn draw_dead_cells(cell_size: f32, (padding_x, padding_y): (f32,f32)) {
    let mut x = padding_x;
    let mut y = padding_y;
    while x + cell_size <= screen_width() - padding_x {
        while y + cell_size <= screen_height() - padding_y {
            draw_rectangle(x, y, cell_size, cell_size, WHITE);
            y += cell_size;
        }
        x += cell_size;
        y = padding_y;
    }
}

fn draw_grid(step: f32, (padding_x, padding_y): (f32,f32)) {
    let mut x = padding_x;
    let mut y = padding_y;
    while (x + step) <= (screen_width() - padding_x) {
        draw_line(x, padding_y, x, screen_height() - padding_y, 1.0, LIGHTGRAY);
        x += step;
    }
    while (y + step) <= (screen_height() - padding_y) {
        draw_line(padding_x, y, screen_width() - padding_x, y, 1.0, LIGHTGRAY);
        y += step;
    }
}
