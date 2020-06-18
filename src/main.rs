//grid width/length, tile count determined by user
//4 different types of cells: Hallways, Filled, Empty cells, Other types(not yet known what they will be) - use an enum for this situation
//Each room will be a struct composed of all the filled cells
//Hallways will hold a strong reference to it's seed room and a weak ref to its child room
//Have a "seed" room
//seed room will spawn with potential* hallways(will be either left, right, up, down hallway)
//the seed will be special and will always spawn with either 3 or 4 hallways(but that doesn't mean those hallways will be successful)
//    *potential means they are not created when the room is created, rather when setting up another room is successful
//rest of the outer tiles will be null
//create new rooms using bfs
//How to create new rooms:
//   go one more direction in which the hallway was created
//   go around in a square to create the new hallway
//   generate random numbers for 3 sides so square will be made
//   side 3 must be greater than or equal to the first side chosen
//   reduce square side by one if the way direction goes + 2(random number between 2 and 4 lets say) is a filled tile -
//   if any of the sides are a length of <= one when it's reduced, don't make the room at all, throw an error
//if new room in certain direction is unsuccessful, the attempt to create a hallway is unsuccessful and removed from room hallway potentials

use num_integer::Roots;
use rand::prelude::*;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Debug;
use std::iter::Iterator;
use std::ops;

trait MiddlePosition {
    fn middle() -> Position;
}

#[derive(Debug)]
pub struct Position {
    row: i64,
    col: i64,
}

impl Position {
    pub fn new(row: i64, col: i64) -> Self {
        Position { row, col }
    }

    pub fn middle_position(row: i64, col: i64) -> Self {
        Position {
            row: row / 2,
            col: col / 2,
        }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;
    fn add(self, _rhs: Position) -> Position {
        Position::new(self.row + _rhs.row, self.col + _rhs.col)
    }
}

#[derive(Debug)]
pub struct CornerPositions {
    //4 corners of the room
    top_left: Position,
    bottom_left: Position,
    top_right: Position,
    bottom_right: Position,
}

impl CornerPositions {
    pub fn new(
        top_left: Position,
        bottom_left: Position,
        top_right: Position,
        bottom_right: Position,
    ) -> Self {
        CornerPositions {
            top_left,
            bottom_left,
            top_right,
            bottom_right,
        }
    }
}
type VecRangeUSize = std::ops::Range<i64>;
#[derive(Debug)]
pub struct Side {
    side_points: VecRangeUSize,
    ref_point: i64,
    start: usize,
    end: usize,
}

impl Side {
    fn new(side_points: VecRangeUSize, ref_point: i64) -> Side {
        Side {
            side_points,
            ref_point,
            start: side_points.start as usize,
            end: side_points.end as usize,
        }
    }
    fn get_side_point(&self, direction: char) -> HallDirection {
        let start = self.side_points.start + 1;
        let end = self.side_points.end - 1;
        let point = rand::thread_rng().gen_range(start, end);
        let hall = match direction {
            'r' => HallwayCell::new(Position::new(point, self.ref_point)),
            'l' => HallwayCell::new(Position::new(point, self.ref_point)),
            'b' => HallwayCell::new(Position::new(self.ref_point, point)),
            't' => HallwayCell::new(Position::new(self.ref_point, point)),
            _ => panic![],
        };
        HallDirection::new(hall, direction)
    }
}

impl Iterator for Side {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.start != self.end - 1 {
            self.start += 1;
            Some(Position::new)
        } else {
            self.count = 0;
            None
        }
    }
}

#[derive(Debug)]
pub struct CardinalSides {
    left: Side,
    right: Side,
    bottom: Side,
    top: Side,
    sides: Vec<Side>,
    count: usize,
}

impl CardinalSides {
    fn new(left: Side, right: Side, bottom: Side, top: Side) -> Self {
        CardinalSides {
            left,
            right,
            bottom,
            top,
            sides: vec![left, right, bottom, top],
            count: 0,
        }
    }
}

impl Iterator for CardinalSides {
    type Item = Side;

    fn next(&mut self) -> Option<Side> {
        if self.count < 4 {
            self.count += 1;
            Some(self.sides[self.count - 1])
        } else {
            self.count = 0;
            None
        }
    }
}

pub struct HallDirection {
    cell: HallwayCell,
    direction: char,
}

impl HallDirection {
    fn new(cell: HallwayCell, direction: char) -> HallDirection {
        HallDirection { cell, direction }
    }
}

#[derive(Debug)]
pub struct Size {
    row_size: i64,
    col_size: i64,
}

impl Size {
    fn new(row_size: i64, col_size: i64) -> Self {
        Size { row_size, col_size }
    }
}

#[derive(Debug)]
pub struct EmptyCell {
    repr: char,
    position: Position,
}

impl EmptyCell {
    fn new(position: Position) -> Self {
        //This should be the same lifetime as the hallway cell
        EmptyCell {
            repr: 'E',
            position,
        }
    }
}

#[derive(Debug)]
pub struct FilledCell {
    repr: char,
    //seeded_from: : eh need to think about this one
    position: Position,
}

#[derive(Debug)]
pub struct HallwayCell {
    repr: char,
    //seeded_from: : eh need to think about this one
    position: Position,
}

impl HallwayCell {
    fn new(position: Position) -> Self {
        //This should be the same lifetime as the hallway cell
        HallwayCell {
            repr: 'H',
            position,
        }
    }
}

#[derive(Debug)]
pub enum Cell {
    Hall(HallwayCell),
    Filled(FilledCell),
    Empty(EmptyCell),
}

#[derive(Debug)]
pub struct Room {
    room_number: i16,
    corner_positions: CornerPositions, //position relative to some overlying feature
    cardinal_sides: CardinalSides,
    hall_cells: Vec<Cell>,
    filled_cells: Vec<Cell>,
    empty_cells: Vec<Cell>,
    // map: RefCell<Weak<Map>>, //This will be a weak reference
}

impl<'a> Room {
    fn create_seed(size: &'a Size, top_left: Position) -> Self {
        let corner_positions = Room::create_corner_positions(size, top_left);
        let sides = Room::create_sides(&corner_positions);
        Room {
            room_number: 0,
            hall_cells: vec![],
            filled_cells: vec![],
            empty_cells: vec![],
            corner_positions: corner_positions,
            cardinal_sides: sides,
        }
    }

    fn create_sides(corners: &CornerPositions) -> CardinalSides {
        let top = Side::new(
            vec![corners.top_left.col..corners.top_right.col],
            corners.top_left.row,
        );
        let bottom = Side::new(
            vec![corners.bottom_left.col..corners.bottom_right.col],
            corners.bottom_left.row,
        );
        let left = Side::new(
            vec![corners.top_left.row..corners.bottom_left.row],
            corners.bottom_left.col,
        );
        let right = Side::new(
            vec![corners.top_left.row..corners.bottom_left.row],
            corners.bottom_right.col,
        );
        CardinalSides::new(left, right, bottom, top)
    }
    //function to get called once rooms are validated
    fn create_corner_positions(size: &'a Size, top_left: Position) -> CornerPositions {
        let top_right = Position::new(top_left.row + size.row_size, top_left.col);
        let bottom_left = Position::new(top_left.row, top_left.col + size.col_size);
        let bottom_right =
            Position::new(top_left.row + size.row_size, top_left.col + size.col_size);
        CornerPositions::new(top_left, bottom_left, top_right, bottom_right)
    }
    //returns a number of halls that could be created
    //picks from the 4 sides, returns the side and the potential hall
    pub fn create_potential_halls(&self, hall_num: i64) -> Vec<HallDirection> {
        let mut rng = rand::thread_rng();
        let mut sides = vec!['r', 'l', 'b', 't'];
        sides.shuffle(&mut rng);
        let num = rng.gen_range(1, 3);
        let mut hall_directions: Vec<HallDirection> = vec![];
        while num != 0 {
            num -= 1;
            let direction = sides.pop();
            let hall_direction = match direction {
                Some('r') => self.cardinal_sides.right.get_side_point('r'),
                Some('l') => self.cardinal_sides.left.get_side_point('l'),
                Some('b') => self.cardinal_sides.bottom.get_side_point('b'),
                Some('t') => self.cardinal_sides.top.get_side_point('t'),
                _ => panic![],
            };
            hall_directions.push(hall_direction)
        }
        hall_directions
    }
    pub fn create_top_room(cell: &HallwayCell) -> Room {
        let top_left = cell.position + Position::new(4, -4);
        let size = Size::new(4, 8);
        Room::create_seed(&size, top_left)
    }

    pub fn create_bottom_room(cell: &HallwayCell) -> Room {
        let top_left = cell.position + Position::new(0, -2);
        let size = Size::new(8, 4);
        Room::create_seed(&size, top_left)
    }

    pub fn create_left_room(cell: &HallwayCell) -> Room {
        let top_left = cell.position + Position::new(2, -8);
        let size = Size::new(8, 4);
        Room::create_seed(&size, top_left)
    }

    pub fn create_right_room(cell: &HallwayCell) -> Room {
        let top_left = cell.position + Position::new(2, 0);
        let size = Size::new(5, 2);
        Room::create_seed(&size, top_left)
    }
}

#[derive(Debug)]
pub struct Maze<'a> {
    rooms: Vec<Room>,
    map_size: &'a Size,
    grid: Vec<Vec<Cell>>,
}

impl<'a> Maze<'a> {
    pub fn new(map_size: &'a Size) -> Self {
        let empty_maze: Vec<Vec<Cell>> = Maze::empty_cell_vecs(map_size);
        Maze {
            rooms: vec![],
            map_size,
            grid: empty_maze,
        }
    }

    pub fn empty_cell_vecs(map_size: &Size) -> Vec<Vec<Cell>> {
        let mut grid: Vec<Vec<Cell>> = vec![];
        let mut row: Vec<Cell>;
        let mut cell: EmptyCell;
        let mut pos: Position; // we want to copy these values into some grid

        for i in 0..map_size.row_size {
            row = vec![];
            for j in 0..map_size.col_size {
                pos = Position::new(i, j);
                cell = EmptyCell::new(pos);
                row.push(Cell::Empty(cell));
            }
            grid.push(row);
        }
        grid
    }

    //intermediary rooms, may have many
    //accepts a side and hallway
    //the side determines the direction of the next room, yada yada
    //the create functions DO NOT place and need to be validated
    fn create_hallway_room(&self, hall_direction: &HallDirection) -> Room {
        match hall_direction.direction {
            'l' => Room::create_left_room(&hall_direction.cell),
            'r' => Room::create_right_room(&hall_direction.cell),
            't' => Room::create_top_room(&hall_direction.cell),
            'b' => Room::create_bottom_room(&hall_direction.cell), //this whole thing could be rewritten as an enum, lets do that instead...
        }
    }

    //seed room, only one, doesn't need to be validated
    fn create_seed_room(&self, room_size: &Size) -> Room {
        let position = Position::middle_position(self.map_size.row_size, self.map_size.col_size);
        let room_area = (self.map_size.row_size * self.map_size.col_size).sqrt();
        let rect_size = Size::new(15, 10);
        Room::create_seed(&rect_size, position)
    }

    fn place_room(&mut self, room: &Room) {}

    //end rooms
    fn create_ending_room(&self) {}

    //validates that no other rooms are on the present tile
    //the room is passed into with already pre-determined sides with ranges...
    //the room checks every single tile in the sides arrays and returns
    //a boolean if the sides check out.
    fn validate_room_placement(&self, room: &Room, hall_cell: HallwayCell) -> bool {
        for side in room.cardinal_sides {
            //need to make iterators of these
            for position in side {
                if hall_cell.position == position {
                    continue;
                }
                let cell_type = self.grid[cell.position.row][cell.position.col];
                if cell_type == Cell::Filled {
                    return false;
                }
            }
        }
        return true;
    }
    //where all rooms are added
    pub fn add_rooms(&self, room_size: &Size, variation: i64, room_count: i64) {
        let mut current_room = self.create_seed_room(room_size);
        //while rooms are still available to make
        let room_queue: VecDeque<Room> = VecDeque::new();
        room_queue.push_back(current_room);
        while !room_queue.is_empty() {
            current_room = room_queue.pop_front().unwrap(); //I would like to panic :)
            self.place_room(&current_room);
            //this portion is the potential part of the map that isn't decided until EVERYTHING checks out
            let halls_with_directions = current_room.create_potential_halls(4);
            for hall_with_direction in &halls_with_directions {
                let room = self.create_hallway_room(hall_with_direction);
                let is_valid_room = self.validate_room_placement(&room, hall_with_direction.cell);
                if is_valid_room {
                    room_count -= 1;
                }
                if room_count == 0 {
                    break;
                }
                if room_count != 0 && !is_valid_room {
                    continue;
                }
                room_queue.push_back(room);
            }
        }

        //side Note:  Rooms will always be considered Ending rooms first and then changed later if
        //the while loop continues, that way I don't have to think about it later :))))))
    }
}

fn main() {
    let size = Size::new(255, 255);
    let position = Position::new(0, 0);
    let rect_size = Size::new(15, 10);
    let room = Room::create_seed(&rect_size, position);
    let maze = Maze::new(&size);
}
