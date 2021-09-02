use crate::types::Facing;

use super::action::Action;

pub struct Actor {
    pub action: Action,
    pub facing: Facing,
}

impl Default for Actor {
    fn default() -> Self {
        Self { action: Default::default(), facing: Default::default() }
    }
}