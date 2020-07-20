#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
            Cell::Empty => 'X',
            Cell::Corner => 'C',
            Cell::MainRoom => 'O',
            Cell::LeftSide => 'L',
            Cell::RightSide => 'R',
            Cell::TopSide => 'T',
            Cell::BottomSide => 'B',
        }
    }
}
