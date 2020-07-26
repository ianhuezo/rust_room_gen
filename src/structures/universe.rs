use super::cells::Cell;
use super::globals::*;
use super::rooms::Room;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::*;
use std::io::prelude::*;
use std::rc::Rc;

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
    pub fn generate_rooms(&mut self, mut room_number: usize) {
        let mut current_room = self.create_starting_room();
        let mut next_room = self.create_starting_room();
        let sides = vec![
            Cell::TopSide,
            Cell::LeftSide,
            Cell::BottomSide,
            Cell::RightSide,
        ];
        let mut side_direction = sides.choose(&mut rand::thread_rng()).unwrap();
        let mut hall_position = current_room.get_random_cell_position_on_side(*side_direction);
        let mut queue: VecDeque<Rc<Position>> = VecDeque::new();
        queue.push_back(hall_position);
        while room_number >= 0 {
            hall_position = match queue.pop_front() {
                Some(v) => v,
                None => break,
            };
            self.place_room(&current_room);
            side_direction = &current_room.cells[&hall_position];
            next_room = match side_direction {
                Cell::LeftSide => self.create_left_room(hall_position),
                _ => break,
            };
            self.place_room(&next_room);
            room_number -= 1;
            break;
        }
    }

    //this can be generic to any room with a callback, will refactor to it
    fn create_left_room(&mut self, position: Rc<Position>) -> Room {
        let room = Room::create_left_room(
            position,
            &Size {
                width: 8,
                height: 6,
            },
            Position::new(self.universe_size, self.universe_size),
            &self.cells,
        );
        match room {
            Some(room) => room,
            None => panic!("Room cannot be none"),
        }
    }

    pub fn create_starting_room(&mut self) -> Room {
        let position_range = self.random_valid_initial_position(5, 7);
        Room::new(
            position_range.start,
            position_range.stop,
            Position::new(self.universe_size, self.universe_size),
        )
        .unwrap()
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

    fn place_room(&mut self, room: &Room) {
        //cells are copied rather than referenced b/c I plan to make room rc eventually
        for (position, cell) in &room.cells {
            *self.cells.get_mut(position).unwrap() = **cell;
        }
    }

    fn place_hall(&mut self, position: &Position) {
        //cells are copied rather than referenced b/c I plan to make room rc eventually
        *self.cells.get_mut(position).unwrap() = Cell::Hall;
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
