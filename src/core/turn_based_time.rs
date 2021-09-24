use crate::actors::Activity;
use bevy_ecs::prelude::*;
use std::cmp::Ordering;

pub struct TimeProgressionEvent {
    pub delta_time: i32,
}

#[derive(Debug, Default)]
pub struct TurnBasedTime {
    pub time: i32,
}

fn order_by_time_left<'r, 's>(first: &'r &Activity, second: &'s &Activity) -> Ordering {
    first.time_to_complete.cmp(&second.time_to_complete)
}

pub fn advance_time(
    mut time: ResMut<TurnBasedTime>,
    mut time_event_writer: EventWriter<TimeProgressionEvent>,
    activities: Query<&Activity>,
) {
    if let Some(shortest_activity) = activities.iter().min_by(order_by_time_left) {
        time.time += shortest_activity.time_to_complete;
        time_event_writer.send(TimeProgressionEvent {
            delta_time: shortest_activity.time_to_complete,
        });
        // println!("Progressing time by {}", shortest_activity.time_to_complete);
    }
}
