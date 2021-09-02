use crate::constants::facings::*;

use super::action::Action;

pub const MOVE_NORTH_WEST: Action = Action::Move(NORTH_WEST);
pub const MOVE_NORTH: Action = Action::Move(NORTH);
pub const MOVE_NORTH_EAST: Action = Action::Move(NORTH_EAST);
pub const MOVE_EAST: Action = Action::Move(EAST);
pub const MOVE_SOUTH_EAST: Action = Action::Move(SOUTH_EAST);
pub const MOVE_SOUTH: Action = Action::Move(SOUTH);
pub const MOVE_SOUTH_WEST: Action = Action::Move(SOUTH_WEST);
pub const MOVE_WEST: Action = Action::Move(WEST);
pub const MOVE_WAIT: Action = Action::Move(IDLE);