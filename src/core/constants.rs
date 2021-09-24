use ultraviolet as uv;

use super::types::Facing;

const COS_PI: f32 = 0.0;
const SIN_PI: f32 = 1.0;
const COS_PI_OVER_4: f32 = std::f32::consts::FRAC_1_SQRT_2;
const COS_PI_OVER_8: f32 = 0.923_879_5;
const SIN_PI_OVER_8: f32 = 0.382_683_4;

pub const NORTH: Facing = uv::Rotor2::new(COS_PI, uv::Bivec2::new(-SIN_PI));
pub const NORTH_EAST: Facing = uv::Rotor2::new(SIN_PI_OVER_8, uv::Bivec2::new(-COS_PI_OVER_8));
pub const EAST: Facing = uv::Rotor2::new(COS_PI_OVER_4, uv::Bivec2::new(-COS_PI_OVER_4));
pub const SOUTH_EAST: Facing = uv::Rotor2::new(COS_PI_OVER_8, uv::Bivec2::new(-SIN_PI_OVER_8));
pub const SOUTH: Facing = uv::Rotor2::new(SIN_PI, uv::Bivec2::new(COS_PI));
pub const SOUTH_WEST: Facing = uv::Rotor2::new(-COS_PI_OVER_8, uv::Bivec2::new(-SIN_PI_OVER_8));
pub const WEST: Facing = uv::Rotor2::new(-COS_PI_OVER_4, uv::Bivec2::new(-COS_PI_OVER_4));
pub const NORTH_WEST: Facing = uv::Rotor2::new(-SIN_PI_OVER_8, uv::Bivec2::new(-COS_PI_OVER_8));
