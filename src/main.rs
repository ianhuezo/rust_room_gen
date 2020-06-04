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

use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
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
type VecRangeUSize = std::vec::Vec<std::ops::Range<usize>>;
pub struct Side {
    side_points: VecRangeUSize,
    ref_point: usize,
    transformed: bool, // transforms rows to cols, cols to rows
}

// impl Iterator for Side {
//     type Item = Position;
//     // Here, we define the sequence using `.curr` and `.next`.
//     // The return type is `Option<T>`:
//     //     * When the `Iterator` is finished, `None` is returned.
//     //     * Otherwise, the next value is wrapped in `Some` and returned.
//     fn next(&mut self) -> Option<Position> {
//         let mut side_points = self.side_points.iter();

//         match side_points.next() {
//             Some(index) if self.transformed => Position::new(row: self.ref_point, index),
//             Some(index) =>  Position::new(row: index, col: self.ref_point),
//         };
//         let val = side_points.next();
//         if self.transformed{
//             let position = Position::new(val, 1);
//             Some(position)
//         }
//         else{
//             Some(Position::new(row: ref_point, col: side_points.next()))
//         }
//     }

#[derive(Debug)]
pub struct CardinalSides {
    left: (VecRangeUSize, usize),
    right: (VecRangeUSize, usize),
    bottom: (usize, VecRangeUSize),
    top: (usize, VecRangeUSize),
}

impl CardinalSides {
    fn new(
        left: (VecRangeUSize, usize),
        right: (VecRangeUSize, usize),
        bottom: (usize, VecRangeUSize),
        top: (usize, VecRangeUSize),
    ) -> Self {
        CardinalSides {
            left,
            right,
            bottom,
            top,
        }
    }
}

#[derive(Debug)]
pub struct Size {
    row_size: usize,
    col_size: usize,
}

impl Size {
    fn new(row_size: usize, col_size: usize) -> Self {
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
struct Room {
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
        let top = (
            corners.top_left.row,
            vec![corners.top_left.col..corners.top_right.col],
        );
        let bottom = (
            corners.bottom_left.row,
            vec![corners.bottom_left.col..corners.bottom_right.col],
        );
        let left = (
            vec![corners.top_left.row..corners.bottom_left.row],
            corners.bottom_left.col,
        );
        let right = (
            vec![corners.top_left.row..corners.bottom_left.row],
            corners.bottom_right.col,
        );
        CardinalSides::new(left, right, bottom, top)
    }

    // fn spawn_potential_hall_cells(&self) -> Vec<HallwayCell> {
    //     let halls = filte
    // }

    fn create_corner_positions(size: &'a Size, top_left: Position) -> CornerPositions {
        let top_right = Position::new(top_left.row + size.row_size, top_left.col);
        let bottom_left = Position::new(top_left.row, top_left.col + size.col_size);
        let bottom_right =
            Position::new(top_left.row + size.row_size, top_left.col + size.col_size);
        CornerPositions::new(top_left, bottom_left, top_right, bottom_right)
    }
}

#[derive(Debug)]
pub struct Maze<'a> {
    rooms: Vec<Room>,
    map_size: &'a Size,
    all_cells: Vec<Vec<Cell>>,
}

impl<'a> Maze<'a> {
    pub fn new(map_size: &'a Size, is_populated: bool) -> Self {
        let empty_maze: Vec<Vec<Cell>> = Maze::empty_cell_vecs(map_size);
        Maze {
            rooms: vec![],
            map_size,
            all_cells: empty_maze,
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

    pub fn in_bounds(&self) {}

    pub fn add_room(&self) {}
}

fn main() {
    let size = Size::new(255, 255);
    let position = Position::new(0, 0);
    let rect_size = Size::new(15, 10);
    let room = Room::create_seed(&rect_size, position);
    let maze = Maze::new(&size, false);
}
