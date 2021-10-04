use bevy::math::Quat;

use crate::core::constants::PI;

use super::{Cardinal, Direction, Real};

pub type Facing = Quat;

impl From<Cardinal> for Facing {
    fn from(val: Cardinal) -> Self {
        match val {
            Cardinal::North => Quat::from_rotation_z(4.0 * PI / 4.0),
            Cardinal::NorthEast => Quat::from_rotation_z(3.0 * PI / 4.0),
            Cardinal::East => Quat::from_rotation_z(2.0 * PI / 4.0),
            Cardinal::SouthEast => Quat::from_rotation_z(1.0 * PI / 4.0),
            Cardinal::South => Quat::from_rotation_z(0.0 * PI / 4.0),
            Cardinal::SouthWest => Quat::from_rotation_z(7.0 * PI / 4.0),
            Cardinal::West => Quat::from_rotation_z(6.0 * PI / 4.0),
            Cardinal::NorthWest => Quat::from_rotation_z(5.0 * PI / 4.0),
        }
    }
}

impl From<Direction> for Facing {
    fn from(direction: Direction) -> Self {
        let cardinal: Cardinal = direction.into();
        cardinal.into()
    }
}
