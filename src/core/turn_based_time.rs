use crate::actor::Activity;
use bevy_ecs::prelude::*;
use std::cmp::Ordering;

pub struct TimeProgressionEvent {
    pub delta_time: i32,
}

#[derive(Debug, Default)]
pub struct TurnBasedTime {
    pub time: i32,
}

fn order_by_time_left<'r, 's>(activity1: &'r &Activity, activity2: &'s &Activity) -> Ordering {
    let delta = activity1.time_to_complete - activity2.time_to_complete;
    if 0 > delta {
        Ordering::Less
    } else if 0 < delta {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
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
