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
        for y in 0..universe_size - 1 {
            for x in 0..universe_size - 1 {
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
            self.place_hall(Position::new(hall_position.x, hall_position.y));
            let start_and_stop_room_positions =
                self.create_start_stop_ranges(hall_position, side_direction);
            next_room = Room::new(
                &start_and_stop_room_positions.start,
                &start_and_stop_room_positions.stop,
                Position::new(self.universe_size, self.universe_size),
            )
            .unwrap();
            self.place_room(&next_room);
            room_number -= 1;
            break;
        }
    }

    fn is_valid_room(&self, positions: &PositionRange) -> bool {
        let (start, stop) = (&positions.start, &positions.stop);
        for val in start.y..stop.y {
            let start_entry = self.cells[&Position::new(start.x, val)];
            let stop_entry = self.cells[&Position::new(stop.x, val)];
            match start_entry {
                Cell::Empty => true,
                Cell::LeftSide | Cell::RightSide | Cell::TopSide | Cell::BottomSide => {
                    return false
                }
                _ => return false,
            };
            //just make this into a function with cells to validate...
            match stop_entry {
                Cell::Empty => true,
                Cell::LeftSide
                | Cell::RightSide
                | Cell::TopSide
                | Cell::BottomSide
                | Cell::Hall
                | Cell::MainRoom => return false,
                _ => return false,
            };
        }
        true
    }

    //this can be generic to any room with a callback, will refactor to it
    fn create_start_stop_ranges(
        &mut self,
        position: Rc<Position>,
        cell_type: &Cell,
    ) -> PositionRange {
        Room::create_start_and_stop_positions(
            position,
            &Size {
                width: thread_rng().gen_range(5, 10),
                height: thread_rng().gen_range(5, 10),
            },
            cell_type,
        )
    }

    pub fn create_starting_room(&mut self) -> Room {
        let position_range = self.random_valid_initial_position(5, 7);
        Room::new(
            &position_range.start,
            &position_range.stop,
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
        let threshold = 5;
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
            if *self.cells.get_mut(position).unwrap() == Cell::Empty {
                *self.cells.get_mut(position).unwrap() = **cell;
            }
        }
    }

    fn place_hall(&mut self, position: Position) {
        //cells are copied rather than referenced b/c I plan to make room rc eventually
        *self.cells.get_mut(&position).unwrap() = Cell::Hall;
    }

    pub fn create_cells_txt(&self, file_name: &str) {
        let mut file = File::create(file_name).expect("Unable to create file");

        for y in 0..self.universe_size - 1 {
            for x in 0..self.universe_size - 1 {
                let position = Position::new(x, y);
                let cell = &self.cells[&position];
                write!(file, "{} ", cell.to_char());
            }
            write!(file, "\n");
        }
    }
}
