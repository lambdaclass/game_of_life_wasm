use macroquad::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
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
    width: i32,
    height: i32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: i32, height: i32) -> Universe {
        Universe {
            width: width,
            height: height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }
    
    pub fn randomize(&mut self) {
        for i in 0..self.cells.len() {
            self.cells[i] = if macroquad::rand::rand() > (u32::MAX / 2) {Cell::Alive} else {Cell::Dead};
        }
    }

    // translates x,y position in a grid to the vector position
    pub fn grid_pos(&self, x: i32, y: i32) -> i32 {
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
                        if  x + i < 0 || 
                            x + i >= self.width || 
                            y + j < 0 ||
                            y + j >= self.height {
                            continue;
                        }
                        
                        let neigh_pos = self.grid_pos(x +i, y+j);
                        match self.cells[neigh_pos as usize] {
                            Cell::Alive => alive_neighbours += 1,
                            Cell::Dead => {}
                        }
                        
                }

                grid_copy[cell_pos as usize] = self.cells[cell_pos as usize].update(alive_neighbours);
                
            }
        } 
        self.cells = grid_copy;
    }

}

#[macroquad::main(window_configuration)]
async fn main() {
    let cell_size = 20.0;
    clear_background(RED);
    let mut universe = Universe::new((screen_width() / cell_size) as i32, (screen_height() / cell_size) as i32);
    universe.randomize();
    loop {
        universe.update();
        let padding = get_board_padding(cell_size);
        render_cells(cell_size, padding, &universe);
        draw_grid(cell_size, padding);
        next_frame().await;
        // std::thread::sleep(std::time::Duration::from_millis(100));
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

fn render_cells(cell_size: f32, (padding_x, padding_y): (f32,f32), universe: &Universe) {
    let mut x = padding_x;
    let mut y = padding_y;
    
    while x + cell_size <= screen_width() - padding_x {
        while y + cell_size <= screen_height() - padding_y {
            let x_grid = (x / cell_size) as i32;
            let y_grid = (y / cell_size) as i32;
            let cell_position = universe.grid_pos(x_grid, y_grid);
            let cell_color = match universe.cells[cell_position as usize] {
                Cell::Dead => WHITE,
                Cell::Alive => BLACK,
            };
            draw_rectangle(x, y, cell_size, cell_size, cell_color);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test01_all_cells_are_dead_when_the_universe_starts() {
        let cell_size = 20.0;
        let width = 800.0;
        let height = 600.0;
        let universe = Universe::new((width / cell_size) as i32, (height / cell_size) as i32);

        for cell in universe.cells.iter() {
            assert_eq!(*cell, Cell::Dead);
        }
    }

    #[test]
    fn test02_living_cell_with_fewer_than_two_neighbours_dies() {
        let cell_without_neighbours = Cell::Alive;
        let cell_with_one_neighbour = Cell::Alive;
        
        let cell_without_neighbours = cell_without_neighbours.update(0);
        let cell_with_one_neighbour = cell_with_one_neighbour.update(1);
        
        assert_eq!(cell_without_neighbours, Cell::Dead);
        assert_eq!(cell_with_one_neighbour, Cell::Dead);
    }

    #[test]
    fn test03_living_cell_with_two_or_three_neighbours_stay_alive() {
        let cell_without_neighbours = Cell::Alive;
        let cell_with_one_neighbour = Cell::Alive;
        
        let cell_without_neighbours = cell_without_neighbours.update(2);
        let cell_with_one_neighbour = cell_with_one_neighbour.update(3);
        
        assert_eq!(cell_without_neighbours, Cell::Alive);
        assert_eq!(cell_with_one_neighbour, Cell::Alive);
    }

    #[test]
    fn test04_living_cell_with_more_than_three_neighbours_dies() {
        let cell_without_neighbours = Cell::Alive;
        let cell_with_one_neighbour = Cell::Alive;
        
        let cell_without_neighbours = cell_without_neighbours.update(4);
        let cell_with_one_neighbour = cell_with_one_neighbour.update(5);
        
        assert_eq!(cell_without_neighbours, Cell::Dead);
        assert_eq!(cell_with_one_neighbour, Cell::Dead);
    }

    #[test]
    fn test05_dead_cell_with_exactly_three_neighbours_becomes_alive() {
        let cell_with_three_neighbours = Cell::Dead;
        
        let cell_with_three_neighbours = cell_with_three_neighbours.update(3);
        
        assert_eq!(cell_with_three_neighbours, Cell::Alive);
    }

    #[test]
    fn test06_dead_cell_with_more_or_less_than_three_neighbours_stay_dead() {
        let cell_with_two_neighbours = Cell::Dead;
        let cell_with_four_neighbour = Cell::Dead;

        let cell_with_two_neighbours = cell_with_two_neighbours.update(2);
        let cell_with_four_neighbour = cell_with_four_neighbour.update(4);

        assert_eq!(cell_with_two_neighbours, Cell::Dead);
        assert_eq!(cell_with_four_neighbour, Cell::Dead);
    }
}