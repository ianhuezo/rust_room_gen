use super::cells::Cell;
use super::globals::*;
use rand::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Room {
    sides: HashMap<Rc<Cell>, RefCell<Vec<Rc<Position>>>>, //I want to access the room either way, both with strong references
    pub cells: HashMap<Rc<Position>, Rc<Cell>>,
}

impl Room {
    //Room always start left to right, top to bottom
    pub fn new(start: &Position, stop: &Position, max_position: Position) -> Option<Self> {
        let bad_positions = Self::greater_than_max(&start, &stop, &max_position)
            || Self::greater_than_stop(&start, &stop)
            || Self::less_than_zero(&start, &stop);
        match bad_positions {
            true => None,
            false => Some(Self::create_generic_room(&start, &stop)),
        }
    }

    fn create_generic_room(start: &Position, stop: &Position) -> Self {
        let mut cells: HashMap<Rc<Position>, Rc<Cell>> = HashMap::new();
        let mut sides: HashMap<Rc<Cell>, RefCell<Vec<Rc<Position>>>> = HashMap::new();
        for y in start.y..stop.y {
            for x in start.x..stop.x {
                let current = Rc::new(Position::new(x, y));
                let cell_type = Rc::new(Self::apply_appropriate_cell_type(&current, &start, &stop));
                cells.insert(Rc::clone(&current), Rc::clone(&cell_type));
                sides
                    .entry(Rc::clone(&cell_type))
                    .or_insert(RefCell::new(Vec::new()))
                    .get_mut()
                    .push(Rc::clone(&current));
            }
        }
        Room {
            sides,
            cells: cells,
        }
    }
    fn offset_y_position(position: Rc<Position>, room_size: &Size) -> (i64, i64) {
        let y = position.y - 1;
        let z = thread_rng().gen_range(0, room_size.height);
        let start = y - z;
        let stop = y + room_size.height - z;
        (start, stop)
    }

    pub fn create_left_or_right_start_and_stop_positions(
        position: Rc<Position>,
        room_size: &Size,
        max_position: Position,
    ) -> PositionRange {
        let stop_x = position.x + 1;
        let start_x = stop_x - room_size.width;
        let (start_y, stop_y) = Self::offset_y_position(position, room_size);
        let start = Position::new(start_x, start_y);
        let stop = Position::new(stop_x, stop_y);
        PositionRange { start, stop }
    }
    pub fn get_random_cell_position_on_side(&self, cell: Cell) -> Rc<Position> {
        let side_cell = match cell {
            Cell::LeftSide | Cell::RightSide | Cell::TopSide | Cell::BottomSide => cell,
            _ => panic!("Invalid cell type, only sides can be chosen"),
        };
        let position = self.sides[&Rc::new(side_cell)].borrow_mut();
        if position.len() > 1 {
            let random_index = thread_rng().gen_range(0, position.len() - 1);
            Rc::clone(&position[random_index])
        } else {
            Rc::clone(&position[0])
        }
    }

    fn greater_than_stop(start: &Position, stop: &Position) -> bool {
        start.x >= stop.x || start.y >= stop.y
    }

    fn greater_than_max(start: &Position, stop: &Position, max: &Position) -> bool {
        start.x >= max.x || start.y >= max.y || stop.x >= max.x || stop.y >= max.y
    }

    fn less_than_zero(start: &Position, stop: &Position) -> bool {
        start.x < 0 || start.y < 0 || stop.x < 0 || stop.y < 0
    }

    fn is_top(current: &Position, start: &Position) -> bool {
        current.y == start.y
    }
    fn is_left(current: &Position, start: &Position) -> bool {
        current.x == start.x
    }
    fn is_right(current: &Position, stop: &Position) -> bool {
        current.x == stop.x - 1
    }
    fn is_bottom(current: &Position, stop: &Position) -> bool {
        current.y == stop.y - 1
    }
    fn apply_appropriate_cell_type(current: &Position, start: &Position, stop: &Position) -> Cell {
        //I apply all booleans first so all of them get equal treatment
        let is_top = Self::is_top(current, start);
        let is_right = Self::is_right(current, stop);
        let is_left = Self::is_left(current, start);
        let is_bottom = Self::is_bottom(current, stop);
        let is_corner = is_top && is_right
            || is_top && is_left
            || is_bottom && is_left
            || is_bottom && is_right;
        if is_corner {
            Cell::Corner
        } else if is_left {
            Cell::LeftSide
        } else if is_right {
            Cell::RightSide
        } else if is_top {
            Cell::TopSide
        } else if is_bottom {
            Cell::BottomSide
        } else {
            Cell::MainRoom
        }
    }
}
