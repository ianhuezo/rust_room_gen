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

use std::fmt::Debug;

#[derive(Debug)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

#[derive(Debug)]
pub struct AreaPosition {
    //4 corners of the room
    top_left: Position,
    bottom_left: Position,
    top_right: Position,
    bottom_right: Position,
}

impl AreaPosition {
    pub fn new(
        top_left: Position,
        bottom_left: Position,
        top_right: Position,
        bottom_right: Position,
    ) -> Self {
        AreaPosition {
            top_left,
            bottom_left,
            top_right,
            bottom_right,
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
    global_position: AreaPosition, //position relative to some overlying feature
    hall_cells: Vec<Cell>,
    filled_cells: Vec<Cell>,
    empty_cells: Vec<Cell>,
    cardinal_bounds: [bool; 4],
    corner_bounds: [bool; 4],
    // map: RefCell<Weak<Map>>, //This will be a weak reference
}

impl<'a> Room {
    fn create_seed(size: &'a Size, top_left: Position) -> Self {
        top_left.row 
        Room {
            room_number: 10,
            hall_cells: vec![],
            filled_cells: vec![],
            empty_cells: vec![],
        }
    }
    fn create_halls_from_room(room: &Room) {
        let halls = &room.hall_cells;
    }
}

#[derive(Debug)]
pub struct Maze<'a> {
    rooms: Vec<Room>,
    map_size: &'a Size,
    all_cells: Vec<Vec<Cell>>,
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

impl<'a> Maze<'a> {
    pub fn new(map_size: &'a Size, is_populated: bool) -> Self {
        let empty_maze: Vec<Vec<Cell>> = empty_cell_vecs(map_size);
        Maze {
            rooms: vec![],
            map_size,
            all_cells: empty_maze,
        }
    }

    pub fn in_bounds(&self) {}

    pub fn add_room(&self) {}
}

fn main() {
    let size = Size::new(255, 255);
    let maze = Maze::new(&size, false);
}
