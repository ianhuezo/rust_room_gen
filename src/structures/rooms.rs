use super::cells::Cell;
use super::globals::Position;
use std::collections::HashMap;

pub struct Room {
    start: Position,
    stop: Position,
    pub cells: HashMap<Position, Cell>, //Make a reference
}

impl Room {
    //Room always start left to right, top to bottom
    pub fn new(start: Position, stop: Position) -> Self {
        let mut cells: HashMap<Position, Cell> = HashMap::new();
        let mut sides: HashMap<Cell, Vec<Position>> = HashMap::new();
        for y in start.y..stop.y {
            for x in start.x..stop.x {
                let current = Position::new(x, y);
                let cell_type = Self::apply_appropriate_cell_type(&current, &start, &stop);
                cells.insert(current, cell_type);
            }
        }
        Room {
            start: start,
            stop: stop,
            cells: cells,
        }
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
