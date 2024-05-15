use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const CELL_SIZE: usize = 10;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

struct Universe {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Universe {
    fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut grid = vec![vec![Cell::Dead; width]; height];
        for row in 0..height {
            for col in 0..width {
                if rng.gen_range(0..2) == 0 {
                    grid[row][col] = Cell::Alive;
                }
            }
        }
        Universe { grid, width, height }
    }

    fn tick(&mut self) {
        let mut new_grid = self.grid.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let live_neighbors = self.live_neighbor_count(row, col);
                new_grid[row][col] = match (self.grid[row][col], live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.grid = new_grid;
    }

    fn live_neighbor_count(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for delta_row in [-1, 0, 1].iter().cloned() {
            for delta_col in [-1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row as isize + delta_row).rem_euclid(self.height as isize) as usize;
                let neighbor_col = (col as isize + delta_col).rem_euclid(self.width as isize) as usize;

                if self.grid[neighbor_row][neighbor_col] == Cell::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    fn draw(&self, buffer: &mut Vec<u32>) {
        for row in 0..self.height {
            for col in 0..self.width {
                let color = match self.grid[row][col] {
                    Cell::Alive => 0xFFFFFF, // White for alive cells
                    Cell::Dead => 0x000000,  // Black for dead cells
                };
                for dy in 0..CELL_SIZE {
                    for dx in 0..CELL_SIZE {
                        let x = col * CELL_SIZE + dx;
                        let y = row * CELL_SIZE + dy;
                        if x < WIDTH && y < HEIGHT {
                            buffer[y * WIDTH + x] = color;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let universe_width = WIDTH / CELL_SIZE;
    let universe_height = HEIGHT / CELL_SIZE;

    let mut universe = Universe::new(universe_width, universe_height);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Conway's Game of Life",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        universe.tick();
        universe.draw(&mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
