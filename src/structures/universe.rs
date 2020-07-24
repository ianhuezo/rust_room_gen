use super::cells::Cell;
use super::globals::*;
use super::rooms::Room;
use rand::prelude::*;
use std::collections::HashMap;
use std::fs::*;
use std::io::prelude::*;

pub struct Universe {
    cells: HashMap<Position, Cell>,
    current_room: Option<Box<Room>>,
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
            current_room: None,
            universe_size,
        }
    }
    pub fn generate_rooms(room_number: usize) {}

    pub fn create_starting_room(&mut self) {
        let position_range = self.random_valid_initial_position(5, 7);
        let room = Room::new(position_range.start, position_range.stop);
        self.place_room(&room);
    }

    pub fn random_valid_initial_position(
        &self,
        max_x_vector: i64,
        max_y_vector: i64,
    ) -> PositionRange {
        let middle_universe = self.universe_size / 2 - self.universe_size / 8;
        let start = Position::new(middle_universe, middle_universe);
        let mut rand_gen = thread_rng();
        let threshold = 3;
        let random_x_vector: i64 =
            rand_gen.gen_range(start.x + threshold, start.x + threshold + max_x_vector);
        let random_y_vector: i64 =
            rand_gen.gen_range(start.y + threshold, start.y + threshold + max_y_vector);
        let stop = Position::new(random_x_vector, random_y_vector);
        PositionRange { start, stop }
    }

    pub fn place_room(&mut self, room: &Room) {
        //cells are copied rather than referenced b/c I plan to make room rc eventually
        for (position, cell) in &room.cells {
            *self.cells.get_mut(position).unwrap() = **cell;
        }
        let hall_position = room.place_hall_on_side(Cell::TopSide);
        *self.cells.get_mut(&hall_position).unwrap() = Cell::Hall;
    }

    pub fn create_cells_txt(&self, file_name: &str) {
        let mut file = File::create(file_name).expect("Unable to create file");

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
