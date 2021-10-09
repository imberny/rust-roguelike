use bevy::prelude::Component;

use crate::core::types::Increment;

use super::Action;

#[derive(Debug, Default, Component)]
pub struct Activity {
    pub time_to_complete: Increment,
    pub action: Action,
}
