use ultraviolet as uv;

use super::{Cardinal, Direction, Real};

const COS_PI: Real = 0.0;
const SIN_PI: Real = 1.0;
const COS_PI_OVER_4: Real = std::f32::consts::FRAC_1_SQRT_2;
const COS_PI_OVER_8: Real = 0.923_879_5;
const SIN_PI_OVER_8: Real = 0.382_683_4;

const NORTH: Facing = uv::Rotor2::new(COS_PI, uv::Bivec2::new(SIN_PI));
const NORTH_EAST: Facing = uv::Rotor2::new(SIN_PI_OVER_8, uv::Bivec2::new(-COS_PI_OVER_8));
const EAST: Facing = uv::Rotor2::new(COS_PI_OVER_4, uv::Bivec2::new(-COS_PI_OVER_4));
const SOUTH_EAST: Facing = uv::Rotor2::new(COS_PI_OVER_8, uv::Bivec2::new(-SIN_PI_OVER_8));
const SOUTH: Facing = uv::Rotor2::new(SIN_PI, uv::Bivec2::new(COS_PI));
const SOUTH_WEST: Facing = uv::Rotor2::new(-COS_PI_OVER_8, uv::Bivec2::new(-SIN_PI_OVER_8));
const WEST: Facing = uv::Rotor2::new(-COS_PI_OVER_4, uv::Bivec2::new(-COS_PI_OVER_4));
const NORTH_WEST: Facing = uv::Rotor2::new(-SIN_PI_OVER_8, uv::Bivec2::new(-COS_PI_OVER_8));

pub type Facing = uv::Rotor2;

impl From<Cardinal> for Facing {
    fn from(val: Cardinal) -> Self {
        match val {
            Cardinal::North => NORTH,
            Cardinal::East => EAST,
            Cardinal::South => SOUTH,
            Cardinal::West => WEST,
            Cardinal::NorthWest => NORTH_WEST,
            Cardinal::NorthEast => NORTH_EAST,
            Cardinal::SouthWest => SOUTH_WEST,
            Cardinal::SouthEast => SOUTH_EAST,
        }
    }
}

impl From<Direction> for Facing {
    fn from(direction: Direction) -> Self {
        let cardinal: Cardinal = direction.into();
        cardinal.into()
    }
}
