mod structures;

use structures::universe::Universe;

fn main() {
    let mut universe = Universe::new(64);
    println!("Starting Room Generator...");
    universe.generate_rooms(12);
    universe.create_cells_txt("cells.txt")
}
