mod structures;

use structures::globals::Position;
use structures::rooms::Room;
use structures::universe::Universe;

fn main() {
    let mut universe = Universe::new(64);
    universe.create_starting_room();
    universe.create_cells_txt("cells.txt")
}
