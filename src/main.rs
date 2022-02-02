use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn update(self, neigh_count: u32) -> Cell {
        match self {
            Cell::Dead if neigh_count == 3 => Cell::Alive,
            Cell::Alive if neigh_count == 2 => Cell::Alive,
            Cell::Alive if neigh_count == 3 => Cell::Alive,
            _ => Cell::Dead
        }
    }
}

struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        Universe {
            width: width,
            height: height,
            cells: vec![Cell::Dead; width * height]
        }
    }

    // translates x,y position in a grid to the vector position
    pub fn grid_pos(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn update(&mut self) {
       let mut grid_copy = self.cells.clone();

        let neigh_positions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];

        for x in 0..self.width {
            for y in 0..self.height {
                let cell_pos = self.grid_pos(x, y);
                let mut alive_neighbours = 0;
                
                for (i, j) in neigh_positions.iter() {
                        if  x as i32 + i < 0 || 
                            x as i32 + i >= self.width as i32 || 
                            y as i32 + j < 0 ||
                            y as i32 + j >= self.height as i32 {
                            continue;
                        }
                        
                        let neigh_pos = self.grid_pos((x as i32 +i) as usize, (y as i32+j) as usize);
                        match self.cells[neigh_pos] {
                            Cell::Alive => alive_neighbours += 1,
                            Cell::Dead => {}
                        }
                        
                }

                grid_copy[cell_pos] = self.cells[cell_pos].update(alive_neighbours);
                
            }
        } 
        self.cells = grid_copy;
    }

}

#[macroquad::main(window_configuration)]
async fn main() {
    let cell_size = 20.0;
    clear_background(RED);
    let mut universe = Universe::new((screen_width() / cell_size) as usize, (screen_height() / cell_size) as usize);
    loop {
        universe.update();
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
