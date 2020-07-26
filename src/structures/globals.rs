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

pub struct Size {
    pub width: i64,
    pub height: i64,
}

impl Size {
    pub fn new(width: i64, height: i64) -> Self {
        Size { width, height }
    }
}
