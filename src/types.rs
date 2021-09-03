use crate::constants::facings::SOUTH;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Facing {
    pub x: i32,
    pub y: i32,
}

impl Facing {
    pub const fn constant(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Default for Facing {
    fn default() -> Self {
        SOUTH
    }
}
