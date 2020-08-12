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
    cell_iterations: Vec<HashMap<Position, Cell>>,
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
            cell_iterations: Vec::new(),
            universe_size,
        }
    }
    pub fn generate_rooms(&mut self, mut room_number: i64) {
        //create a starting room
        //have a queue to contain these rooms
        //pop a room out of queue
        //generate the halls from that room
        //try to create each room from each hall generated
        //validate each time a room is created
        //add the newly created room to the queue in the front
        //profit
        let mut current_room = self.create_starting_room();
        let sides = vec![
            Cell::TopSide,
            Cell::LeftSide,
            Cell::BottomSide,
            Cell::RightSide,
        ];
        let mut queue: VecDeque<Room> = VecDeque::new();
        queue.push_back(current_room);

        while room_number >= 0 {
            current_room = match queue.pop_front() {
                Some(v) => v,
                None => break,
            };
            self.place_room(&current_room);
            self.cell_iterations.push(self.cells.clone());
            for room_side in sides.choose_multiple(&mut rand::thread_rng(), 4) {
                let hall_cell = current_room.get_random_cell_position_on_side(*room_side);
                let skipped_side = current_room.get_current_side(*room_side);
                let start_and_stop_room_positions =
                    self.create_start_stop_ranges(Rc::clone(&hall_cell), room_side);
                if self.is_valid_room(&start_and_stop_room_positions, &room_side, &skipped_side) {
                    let next_room = Room::new(
                        &start_and_stop_room_positions.start,
                        &start_and_stop_room_positions.stop,
                        Position::new(self.universe_size, self.universe_size),
                    );
                    if room_number > 1 {
                        match next_room {
                            Some(val) => queue.push_back(val),
                            None => continue,
                        };
                        self.place_hall(&hall_cell);
                        room_number -= 1;
                    }
                }
            }
        }
    }
    fn validate_x_positions(
        &self,
        positions: &PositionRange,
        reference_cell: &Cell,
        skipped_side: &Vec<Rc<Position>>,
    ) -> bool {
        let (start, stop) = (positions.start, positions.stop); //This is copied here, should have better way without copying, idk how to implement yet though
        let mut is_valid = true;
        for val in start.x..stop.x {
            let mut current_position = Position::new(val, stop.y - 1);
            if skipped_side
                .iter()
                .any(|v| **v == Position::new(val, stop.y - 1))
            {
                continue;
            }
            is_valid = current_position.greater_than_zero()
                && current_position.less_than_max(self.universe_size)
                && is_valid
                && match self.cells[&current_position] {
                    Cell::Empty | Cell::Corner => true,
                    _ => false,
                };
            current_position = Position::new(val, start.y);
            if skipped_side.iter().any(|v| **v == current_position) {
                continue;
            }
            is_valid = current_position.greater_than_zero()
                && current_position.less_than_max(self.universe_size)
                && is_valid
                && match self.cells[&current_position] {
                    Cell::Empty | Cell::Corner => true,
                    _ => false,
                }
        }
        is_valid
    }

    fn validate_y_positions(
        &self,
        positions: &PositionRange,
        reference_cell: &Cell,
        skipped_side: &Vec<Rc<Position>>,
    ) -> bool {
        let (start, stop) = (positions.start, positions.stop); //This is copied here, should have better way without copying, idk how to implement yet though
        let mut is_valid = true;
        for val in start.y..stop.y {
            let mut current_position = Position::new(stop.x - 1, val);
            if skipped_side
                .iter()
                .any(|v| **v == Position::new(stop.x - 1, val))
            {
                continue;
            }
            is_valid = is_valid
                && current_position.greater_than_zero()
                && current_position.less_than_max(self.universe_size)
                && match self.cells[&current_position] {
                    Cell::Empty | Cell::Corner => true,
                    _ => false,
                };
            current_position = Position::new(stop.y, val);
            if skipped_side.iter().any(|v| **v == current_position) {
                continue;
            }

            is_valid = is_valid
                && current_position.greater_than_zero()
                && current_position.less_than_max(self.universe_size)
                && match self.cells[&current_position] {
                    Cell::Empty | Cell::Corner => true,
                    _ => false,
                }
        }
        is_valid
    }

    fn is_valid_room(
        &self,
        positions: &PositionRange,
        reference_cell: &Cell,
        skipped_side: &Vec<Rc<Position>>,
    ) -> bool {
        let is_horizontal_valid =
            self.validate_x_positions(positions, reference_cell, skipped_side);
        let is_vertical_valid = self.validate_y_positions(positions, reference_cell, skipped_side);
        is_horizontal_valid && is_vertical_valid
    }

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

    fn place_hall(&mut self, position: &Position) {
        //cells are copied rather than referenced b/c I plan to make room rc eventually
        *self.cells.get_mut(position).unwrap() = Cell::Hall;
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
