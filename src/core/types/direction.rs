use serde::Deserialize;

use super::{Cardinal, Int};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum Direction {
    Forward,
    Right,
    Back,
    Left,
    ForwardLeft,
    ForwardRight,
    BackLeft,
    BackRight,
}

impl From<Cardinal> for Direction {
    fn from(cardinal: Cardinal) -> Self {
        match cardinal {
            Cardinal::North => Self::Forward,
            Cardinal::East => Self::Right,
            Cardinal::South => Self::Back,
            Cardinal::West => Self::Left,
            Cardinal::NorthWest => Self::ForwardLeft,
            Cardinal::NorthEast => Self::ForwardRight,
            Cardinal::SouthWest => Self::BackLeft,
            Cardinal::SouthEast => Self::BackRight,
        }
    }
}

impl From<Int> for Direction {
    fn from(val: Int) -> Self {
        let cardinal: Cardinal = val.into();
        cardinal.into()
    }
}

impl From<Direction> for Int {
    fn from(direction: Direction) -> Self {
        let cardinal: Cardinal = direction.into();
        cardinal.into()
    }
}
