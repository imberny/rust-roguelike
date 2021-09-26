use super::{Direction, Int};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl From<Direction> for Cardinal {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Forward => Self::North,
            Direction::Right => Self::East,
            Direction::Back => Self::South,
            Direction::Left => Self::West,
            Direction::ForwardLeft => Self::NorthWest,
            Direction::ForwardRight => Self::NorthEast,
            Direction::BackLeft => Self::SouthWest,
            Direction::BackRight => Self::SouthEast,
        }
    }
}

impl From<Int> for Cardinal {
    fn from(val: Int) -> Self {
        match val {
            0 => Cardinal::North,
            1 => Cardinal::NorthEast,
            2 => Cardinal::East,
            3 => Cardinal::SouthEast,
            4 => Cardinal::South,
            5 => Cardinal::SouthWest,
            6 => Cardinal::West,
            _ => Cardinal::NorthWest,
        }
    }
}

impl From<Cardinal> for Int {
    fn from(cardinal: Cardinal) -> Self {
        match cardinal {
            Cardinal::North => 0,
            Cardinal::NorthEast => 1,
            Cardinal::East => 2,
            Cardinal::SouthEast => 3,
            Cardinal::South => 4,
            Cardinal::SouthWest => 5,
            Cardinal::West => 6,
            Cardinal::NorthWest => 7,
        }
    }
}
