use macroquad::prelude::*;

const WINDOW_SIZE_X : i32 = 1600;
const WINDOW_SIZE_Y : i32 = 900;

#[derive(Clone, Copy)]
enum CellState {
    Alive(u32),
    Dead(u32),
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Connways GoL - Rule 30".to_owned(),
        window_width: WINDOW_SIZE_X,
        window_height: WINDOW_SIZE_Y,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut cells = vec![CellState::Dead(0); (w/4)*(h/4)];
    let mut buffer = vec![CellState::Dead(0); (w/4)*(h/4)];

    let mut image = Image::gen_image_color((w/4) as u16, (h/4) as u16, WHITE);
    let texture : Texture2D = Texture2D::from_image(&image);

    cells[(w/4) * ((h/4)-1) + (w/4)/2] = CellState::Alive(0);
    buffer[(w/4) * ((h/4)-1) + (w/4)/2] = CellState::Alive(0);

     let hex_colors = [
        color_u8!(113, 28, 145, 255),
        color_u8!(234, 0, 217, 255),
        color_u8!(10, 189, 198, 255),
        color_u8!(19, 62, 124, 255),
        color_u8!(9, 24, 51, 255),
        color_u8!(0, 1, 3, 255),
    ];
    let colors = color_gradient(&hex_colors);

    loop {
        clear_background(BLACK);

        let w = image.width();
        let h = image.height();

        let gol_start = h / 2;

        // update game of life state
        for row in 0..=gol_start {
            for col in 0..w {
                let mut num_neighbours = 0;
                for x in -1i32..=1 {
                    for y in -1i32..=1 {
                        if x == 0 && y == 0 {continue;}
                        if row as i32 + x < 0 || row as i32 + x > gol_start as i32
                        || col as i32 + y < 0 || col as i32 + y >= w as i32 {
                            continue;
                        }

                        let pos = w as i32 * (row as i32 +x) + col as i32 + y;

                        if let CellState::Alive(_) = cells[pos as usize] {
                            num_neighbours += 1;
                        }
                        
                    }
                }

                let new_state = match (cells[(w * row + col) as usize], num_neighbours) {
                    (CellState::Alive(t), 2) | (CellState::Alive(t), 3) => {
                        CellState::Alive(t.saturating_add(1))
                    }
                    (CellState::Dead(_), 3) => CellState::Alive(0),
                    (CellState::Alive(t), _) => CellState::Dead(t.saturating_add(1)),
                    (CellState::Dead(0), _) => CellState::Dead(0),
                    (CellState::Dead(t), _) => CellState::Dead(t.saturating_add(1)),
                };
                
                buffer[(w * row + col) as usize] = new_state;
            }
        }

        // shift rows upwards (ignoring decay)
        for row in (gol_start..h-1).rev() {
            for col in 0..w {
                buffer[row * w + col] = match cells[(row+1) * w + col] {
                    CellState::Alive(t) => CellState::Alive(t.saturating_add(1)),
                    CellState::Dead(t) => CellState::Dead(t)
                }
            }
        }

        // create new bottom row
        // it wraps at the borders
        for col in 0..w {
            let c = col as i32;
            let state_up_cell : (CellState, CellState, CellState) = (
               cells[(h-2) * w + ((c - 1 + w as i32) % (w as i32)) as usize], 
               cells[(h-2) * w + col], 
               cells[(h-2) * w + ((c + 1 + w as i32) % (w as i32)) as usize], 
            );

            // rule 30
            buffer[w * (h-1) + col] = match state_up_cell {
                (CellState::Alive(_), CellState::Alive(_), CellState::Alive(_)) | 
                (CellState::Alive(_), CellState::Alive(_), CellState::Dead(_)) |
                (CellState::Alive(_), CellState::Dead(_), CellState::Alive(_)) |
                (CellState::Dead(_), CellState::Dead(_), CellState::Dead(_)) => CellState::Dead(0),
                _ => CellState::Alive(0)
            }
            //buffer[w * (h-1) + col] = match state_up_cell {
                //(CellState::Alive(_), CellState::Alive(_), CellState::Alive(_)) | 
                //(CellState::Alive(_), CellState::Dead(_), CellState::Dead(_)) |
                //(CellState::Dead(_), CellState::Dead(_), CellState::Dead(_)) => CellState::Dead(0),
                //_ => CellState::Alive(0)
            //}
        }



        for i in 0..buffer.len() {
            cells[i] = buffer[i];

            image.set_pixel(
                (i % w) as u32,
                (i / w) as u32,
                cell_color(buffer[i as usize], &colors)
            );
        }

        texture.update(&image);

        draw_texture_ex(
            texture, 
            0., 
            0., 
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2((w*4) as f32, (h*4) as f32)),
                ..Default::default()
            }
        );

        next_frame().await
    }
}

fn cell_color(cell: CellState, colors: &Vec<Color>) -> Color {
    match cell {
        CellState::Dead(time_alive) => colors[time_alive as usize],
        CellState::Alive(time_alive) => colors[time_alive as usize]
    }
}

fn color_gradient(hex_colors: &[Color]) -> Vec<Color> {
    let mut color_decay_times = Vec::new();
    for i in 0..(hex_colors.len()-1) {
        color_decay_times.push((2 * 8 as u32).saturating_pow(i as u32));
    }

    assert!(hex_colors.len() == (color_decay_times.len() + 1));
    let mut color_list = vec![BLACK];
    for i in 0..(hex_colors.len() - 1) {
        color_list.append(&mut color_range(hex_colors[i], hex_colors[i+1], color_decay_times[i]));
    }
    color_list.push(WHITE);

    color_list
}

fn color_range(c1: Color, c2: Color, num_points: u32) -> Vec<Color> {
    let mut col_range = vec![c1];

    for t_int in 0..=num_points {
        let t = t_int as f32 / num_points as f32;
        col_range.push(Color {
            r: (c2.r - c1.r) * t + c1.r,
            g: (c2.g - c1.g) * t + c1.g,
            b: (c2.b - c1.b) * t + c1.b,
            a: 1.,
        });
    }

    col_range.push(c2);
    col_range
}
