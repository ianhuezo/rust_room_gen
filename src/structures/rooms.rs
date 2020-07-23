use super::cells::Cell;
use super::globals::Position;
use std::cell::RefCell;
use std::collections::hash_map::*;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Room {
    start: Position,
    stop: Position,
    sides: HashMap<Rc<Cell>, RefCell<Vec<Rc<Position>>>>,
    pub cells: HashMap<Rc<Position>, Rc<Cell>>, //Make a reference
}

impl Room {
    //Room always start left to right, top to bottom
    pub fn new(start: Position, stop: Position) -> Self {
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
                    .push(Rc::clone(&current))
            }
        }
        Room {
            start: start,
            stop: stop,
            sides,
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
