use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Cell {
    Hall,
    Empty,
    Corner,
    MainRoom,
    TopSide,
    BottomSide,
    LeftSide,
    RightSide,
}

impl Cell {
    pub fn to_char(&self) -> char {
        match self {
            Cell::Hall => 'H',
            Cell::Empty => ' ',
            Cell::Corner => 'C',
            Cell::MainRoom => '.',
            Cell::LeftSide => 'L',
            Cell::RightSide => 'R',
            Cell::TopSide => 'T',
            Cell::BottomSide => 'B',
        }
    }
}
