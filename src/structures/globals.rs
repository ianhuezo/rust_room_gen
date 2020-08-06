#[derive(PartialEq, Eq, Hash, Debug)]
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

impl PositionRange {
    //scales the perimeter of the square so that all sides are basically reduced to the next inner sides
    pub fn scale_perimeter(&self, scale_factor: i64) -> PositionRange {
        let start_position = &self.start;
        let end_position = &self.stop;
        let scaled_start_position = Position::new(
            start_position.x + scale_factor,
            start_position.y + scale_factor,
        );
        let scaled_stop_position =
            Position::new(end_position.x - scale_factor, end_position.y - scale_factor);
        PositionRange {
            start: scaled_start_position,
            stop: scaled_stop_position,
        }
    }
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
