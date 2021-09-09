use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use rltk::{console, Point};

use crate::{
    actor::{action::MessageType, Action, Activity, Actor, Viewshed},
    initialization::TimeProgressionEvent,
    map::{Map, TileType},
    types::{Facing, Position},
};

fn do_move(pos: &mut Point, viewshed: &mut Viewshed, movement_delta: &Facing, map: &Res<Map>) {
    let destination_idx = map.xy_idx(pos.x + movement_delta.x, pos.y + movement_delta.y);
    let destination_tile = map.tiles[destination_idx];
    if destination_tile != TileType::Wall {
        pos.x = min(79, max(0, pos.x + movement_delta.x));
        pos.y = min(49, max(0, pos.y + movement_delta.y));

        viewshed.dirty = true;
    }
}

pub fn progress_activities(
    mut time_events: EventReader<TimeProgressionEvent>,
    mut activities: Query<&mut Activity>,
) {
    for time_event in time_events.iter() {
        for mut activity in activities.iter_mut() {
            activity.time_to_complete =
                std::cmp::max(0, activity.time_to_complete - time_event.delta_time);
        }
    }
}

pub fn process_move_actions(
    mut commands: Commands,
    map: Res<Map>,
    mut actors: Query<(Entity, &mut Actor, &mut Position, &mut Viewshed, &Activity)>,
) {
    for (entity, mut actor, mut pos, mut viewshed, activity) in actors.iter_mut() {
        if activity.time_to_complete == 0 {
            // console::log("Doing something");
            match activity.action {
                Action::Move(direction) => {
                    do_move(&mut pos, &mut viewshed, &direction, &map);
                }
                Action::Face(direction) => actor.facing = direction,
                Action::Say(message) => match message.kind {
                    MessageType::Insult => console::log("*!!$%$#&^%@"),
                    MessageType::Threaten => console::log("Shouldn't have come here"),
                    MessageType::Compliment => console::log("Lookin' good today!"),
                },
                _ => (),
            }
            commands.entity(entity).remove::<Activity>();
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;

    use crate::{
        actor::{action::Action, Activity, ActorBundle},
        map::{Map, TileType},
        types::Position,
    };

    use super::process_move_actions;

    fn test_map() -> Map {
        Map {
            tiles: vec![TileType::Floor; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed: vec![false; 80 * 50],
            visible: vec![false; 80 * 50],
        }
    }

    #[test]
    fn none_action() {
        let mut world = World::new();
        world.insert_resource(test_map());
        let entity = world.spawn().insert_bundle(ActorBundle::default()).id();
        let mut stage = SystemStage::single(process_move_actions.system());

        // run process_move_action
        stage.run(&mut world);

        // check action is none
        let activity = world.get::<Activity>(entity).unwrap();
        assert_eq!(Action::None, activity.action);
    }

    #[test]
    fn move_action() {
        let mut world = World::new();
        world.insert_resource(test_map());
        let entity = world.spawn().insert_bundle(ActorBundle::default()).id();
        let mut stage = SystemStage::single(process_move_actions.system());

        // run process_move_action
        stage.run(&mut world);

        // check action is consumed
        let activity = world.get::<Activity>(entity).unwrap();
        assert_eq!(Action::None, activity.action);
        let position = world.get::<Position>(entity).unwrap();
        assert_eq!(Position::new(0, 1), *position);
    }
}
