#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        return Position { x, y };
    }
}

pub struct Velocity {
    pub x: i32,
    pub y: i32,
}
