use core::cmp::PartialEq;

///What every side uses as a basis
struct GeneralSide {
    /// The start of the side, always referenced with top and left first, never right or bottom
    start: Position,
    /// The end of the side
    end: Position,
    ///The binded direction of the side for reference
    direction: Direction,
}

pub enum Side {
    LEFT,
    RIGHT,
    BOTTOM,
    TOP,
}

trait InitializeSide {
    fn new(&self, position: Position) -> GeneralSide;
}

impl InitializeSide for Side {
    fn new(&self, start_position: Position, end_position: Position) -> GeneralSide {
        match self {
            Side::LEFT => GeneralSide {
                start: start_position,
                end: end_position,
                direction: Cell::Hall,
            },
            Side::Right => GeneralSide {
                start: start_position,
                end: end_position,
                direction: Side::Right,
            },
            Side::TOP => GeneralSide {
                start: start_position,
                end: end_position,
                direction: Cell::Hall,
            },
            Side::BOTTOM => GeneralSide {
                start: start_position,
                end: end_position,
                direction: Cell::Hall,
            },
        }
    }
}
