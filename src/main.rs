use std::fs::*;
use std::io::prelude::*;
mod structures;

use structures::cells::Cell;
use structures::globals::Position;
use structures::rooms::Room;
use structures::universe::Universe;

fn create_cells_txt(cells: Vec<char>) {
    let mut file = File::create("cells.txt").expect("Unable to create file");
    for cell in &cells {
        write!(file, "{} ", cell);
    }
}

fn main() {
    let start = Position::new(0, 0);
    let stop = Position::new(5, 5);
    let room = Room::new(start, stop);
    let universe = Universe::new(64);
    universe.create_cells_txt("cells.txt")
}
