#[derive(PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }
}

pub struct PositionRange {
    pub start: Position,
    pub stop: Position,
}
