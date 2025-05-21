#[derive(Clone, Copy)]
pub struct Cell {
    pub x: i16,
    pub y: i16,
    pub cell_type: CellType,
}

#[derive(Clone, Copy)]
pub enum CellType {
    LiveCell,
    DeadCell,
}

impl Cell {
    pub fn new(x: i16, y: i16, cell_type: CellType) -> Cell {
        Cell { x, y, cell_type }
    }

    pub fn get_coordinates(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    pub fn get_dead(&self) -> Cell {
        Cell::new(self.x, self.y, CellType::DeadCell)
    }

    pub fn get_live(&self) -> Cell {
        Cell::new(self.x, self.y, CellType::LiveCell)
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            x: 0,
            y: 0,
            cell_type: CellType::DeadCell,
        }
    }
}
