pub struct GeneralCell {
    pub repr: char,
}

pub enum Cell {
    Hall,
    Empty,
    Corner,
    MainRoom,
    Side,
}

impl Cell {
    pub fn to_char(&self) -> char {
        match self {
            Cell::Hall => 'H',
            Cell::Empty => 'X',
            Cell::Corner => 'C',
            Cell::MainRoom => 'O',
            Cell::Side => 'S',
        }
    }
}
