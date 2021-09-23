use ultraviolet as uv;

use super::types::Facing;

pub const NORTH: Facing = uv::Rotor2::new(0.0, uv::Bivec2::new(-1.0));
pub const NORTH_EAST: Facing = uv::Rotor2::new(0.3827, uv::Bivec2::new(-0.9239));
pub const EAST: Facing = uv::Rotor2::new(0.7071, uv::Bivec2::new(-0.7071));
pub const SOUTH_EAST: Facing = uv::Rotor2::new(0.9239, uv::Bivec2::new(-0.3827));
pub const SOUTH: Facing = uv::Rotor2::new(1.0, uv::Bivec2::new(0.0));
pub const SOUTH_WEST: Facing = uv::Rotor2::new(-0.9239, uv::Bivec2::new(-0.3827));
pub const WEST: Facing = uv::Rotor2::new(-0.7071, uv::Bivec2::new(-0.7071));
pub const NORTH_WEST: Facing = uv::Rotor2::new(-0.3827, uv::Bivec2::new(-0.9239));
