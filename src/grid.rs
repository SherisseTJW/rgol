use crate::cell::{Cell, CellType};

const NEIGHBOUR_COORDINATES_OFFSETS: [(i16, i16); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct Grid {
    pub cells: Vec<Cell>,
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let mut cells = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x as i16, y as i16, CellType::DeadCell)); // Assuming Cell::new(x, y)
            }
        }

        Grid {
            cells,
            width,
            height,
        }
        // let cells = vec![Cell::default(); (width * height) as usize];
        //
        // Grid {
        //     cells,
        //     width,
        //     height,
        // }
    }

    pub fn init(&mut self) {
        let mid_x = self.width / 2;
        let mid_y = self.height / 2;
        let middle_cell_idx = (mid_y * self.width + mid_x) as usize;

        let middle_cell: Cell = self.cells[middle_cell_idx];

        self.cells[middle_cell_idx] = middle_cell.get_live();
    }

    pub fn get_all_neighbors(&self, cur_cell: Cell) -> [Option<Cell>; 8] {
        let mut neighbors: [Option<Cell>; 8] = [None; 8];

        for (i, (dx, dy)) in NEIGHBOUR_COORDINATES_OFFSETS.iter().enumerate() {
            let nx = cur_cell.x + dx;
            let ny = cur_cell.y + dy;

            if nx >= 0 && ny >= 0 && (nx as u32) < self.width && (ny as u32) < self.height {
                let index = ny as u32 * self.width + nx as u32;
                neighbors[i] = Some(self.cells[index as usize]);
            }
        }

        neighbors
    }

    pub fn get_num_neighbors_alive(&self, cur_cell: Cell) -> i8 {
        let mut count: i8 = 0;

        let neighbors: [Option<Cell>; 8] = self.get_all_neighbors(cur_cell);
        for neighbor in neighbors.iter().flatten() {
            if let CellType::LiveCell = neighbor.cell_type {
                count += 1;
            }
        }

        println!("Found {} live neighbors", count);

        count
    }

    pub fn get_next_state(&self) -> Grid {
        let mut new_grid: Grid = Grid::new(self.width, self.height);

        for i in 0..self.cells.len() {
            let cur_cell = self.cells[i];
            let num_alive = self.get_num_neighbors_alive(cur_cell);

            let new_cell = match cur_cell.cell_type {
                CellType::LiveCell => {
                    if (num_alive == 2) || (num_alive == 3) {
                        cur_cell.get_live()
                    } else {
                        cur_cell.get_dead()
                    }
                }
                CellType::DeadCell => {
                    if num_alive == 3 {
                        cur_cell.get_live()
                    } else {
                        cur_cell.get_dead()
                    }
                }
            };

            new_grid.cells[i] = new_cell;
        }

        new_grid
    }
}
