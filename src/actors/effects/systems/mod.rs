use bevy_ecs::prelude::*;

use crate::core::TimeProgressionEvent;

use super::Effect;

pub fn progress_effects(
    mut commands: Commands,
    mut time_events: EventReader<TimeProgressionEvent>,
    mut query: Query<(Entity, &mut Effect)>,
) {
    time_events.iter().for_each(|time_event| {
        query.for_each_mut(|(entity, mut effect)| {
            if time_event.delta_time >= effect.time_left {
                effect.time_left = 0;
                commands.entity(entity).despawn();
            } else {
                effect.time_left -= time_event.delta_time;
            }
        });
    })
}
