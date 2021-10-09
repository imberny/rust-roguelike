use std::cmp::Ordering;

use bevy::prelude::*;

use crate::{
    actors::Activity,
    core::{IncrementalClock, TimeIncrementEvent},
};

fn order_by_time_left<'r, 's>(first: &'r &Activity, second: &'s &Activity) -> Ordering {
    first.time_to_complete.cmp(&second.time_to_complete)
}

pub fn advance_time(
    mut clock: ResMut<IncrementalClock>,
    mut time_event_writer: EventWriter<TimeIncrementEvent>,
    activities: Query<&Activity>,
) {
    if let Some(shortest_activity) = activities.iter().min_by(order_by_time_left) {
        clock.time += shortest_activity.time_to_complete;
        time_event_writer.send(TimeIncrementEvent {
            delta_time: shortest_activity.time_to_complete,
        });
        println!("Advancing time");
    }
}
