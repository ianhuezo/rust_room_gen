use super::cells::Cell;
use super::globals::Position;
use std::collections::HashMap;
use std::fs::*;
use std::io::prelude::*;

pub struct Universe {
    cells: HashMap<Position, Cell>,
    universe_size: i64,
}

impl Universe {
    pub fn new(universe_size: i64) -> Self {
        let mut cells: HashMap<Position, Cell> = HashMap::new();
        for y in 0..universe_size {
            for x in 0..universe_size {
                let position = Position::new(x, y);
                cells.insert(position, Cell::Empty);
            }
        }
        Universe {
            cells,
            universe_size,
        }
    }

    pub fn create_cells_txt(&self, file_name: &str) {
        let mut file = File::create("cells.txt").expect("Unable to create file");

        for y in 0..self.universe_size {
            for x in 0..self.universe_size {
                let position = Position::new(x, y);
                let cell = &self.cells[&position];
                write!(file, "{} ", cell.to_char());
            }
            write!(file, "\n");
        }
    }
}
