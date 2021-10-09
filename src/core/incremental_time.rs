use bevy::prelude::Component;

use super::types::Increment;

pub struct TimeIncrementEvent {
    pub delta_time: Increment,
}

#[derive(Debug, Default, Component)]
pub struct IncrementalClock {
    pub time: Increment,
}
