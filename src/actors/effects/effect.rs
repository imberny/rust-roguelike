use bevy::prelude::*;

use crate::core::types::Increment;

#[derive(Debug, Clone, Copy, Component)]
pub struct Effect {
    pub time_left: Increment,
}
