use macroquad::prelude::*;

#[macroquad::main(window_configuration)]
async fn main() {
    loop {
        clear_background(WHITE);
        next_frame().await
    }
}

// Windows size config is ignored in wasm/android
fn window_configuration() -> Conf {
    Conf {
        window_height: 600,
        window_width: 1200,
        fullscreen: false,
        ..Default::default()
    }
}