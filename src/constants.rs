pub mod facings {
    use crate::types::Facing;

    pub const NORTH_WEST: Facing = Facing::constant(-1, -1);
    pub const NORTH: Facing = Facing::constant(0, -1);
    pub const NORTH_EAST: Facing = Facing::constant(1, -1);
    pub const EAST: Facing = Facing::constant(1, 0);
    pub const SOUTH_EAST: Facing = Facing::constant(1, 1);
    pub const SOUTH: Facing = Facing::constant(0, 1);
    pub const SOUTH_WEST: Facing = Facing::constant(-1, 1);
    pub const WEST: Facing = Facing::constant(-1, 0);
}
