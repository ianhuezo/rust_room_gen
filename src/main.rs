mod structures;

use structures::globals::Position;
use structures::rooms::Room;
use structures::universe::Universe;

fn main() {
    let mut universe = Universe::new(64);
    println!("Starting Room Generator...");
    universe.generate_rooms(2);
    universe.create_cells_txt("cells.txt")
}
