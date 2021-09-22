use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use rltk::{console, Point};

use crate::{
    actor::{Action, Activity, Actor},
    core::types::{Facing, Position},
    core::TimeProgressionEvent,
    game_world::{AreaGrid, TileType, Viewshed},
};

fn do_move(pos: &mut Point, viewshed: &mut Viewshed, movement_delta: &Facing, map: &Res<AreaGrid>) {
    let destination_idx = map.xy_idx(pos.x + movement_delta.x, pos.y + movement_delta.y);
    let destination_tile = map.tiles[destination_idx];
    if destination_tile != TileType::Wall {
        pos.x = min(79, max(0, pos.x + movement_delta.x));
        pos.y = min(49, max(0, pos.y + movement_delta.y));

        viewshed.dirty = true;
    }
}

pub fn advance_activities(
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

pub fn process_activities(
    mut commands: Commands,
    map: Res<AreaGrid>,
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
                // Action::Say(message) => match message.kind {
                //     MessageType::Insult => console::log("*!!$%$#&^%@"),
                //     MessageType::Threaten => console::log("Shouldn't have come here"),
                //     MessageType::Compliment => console::log("Lookin' good today!"),
                // },
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
        actor::{Action, Activity, ActorBundle},
        core::{constants::facings, types::Position},
        game_world::{AreaGrid, TileType},
    };

    use super::process_activities;

    fn test_map() -> AreaGrid {
        AreaGrid {
            tiles: vec![TileType::Floor; 80 * 50],
            width: 80,
            height: 50,
            revealed: vec![false; 80 * 50],
            visible: vec![false; 80 * 50],
        }
    }

    #[test]
    fn consume_activity_upon_completion() {
        let mut world = World::new();
        world.insert_resource(test_map());
        let entity = world
            .spawn()
            .insert_bundle(ActorBundle::default())
            .insert(Activity::default())
            .id();

        let mut stage = SystemStage::single(process_activities.system());
        stage.run(&mut world);

        assert!(world.get::<Activity>(entity).is_none());
    }

    #[test]
    fn move_action() {
        let mut world = World::new();
        world.insert_resource(test_map());
        let entity = world
            .spawn()
            .insert_bundle(ActorBundle::default())
            .insert(Activity {
                action: Action::Move(facings::SOUTH),
                ..Default::default()
            })
            .id();

        let mut stage = SystemStage::single(process_activities.system());
        stage.run(&mut world);

        assert!(world.get::<Activity>(entity).is_none());
        let position = world.get::<Position>(entity).unwrap();
        assert_eq!(Position::new(0, 1), *position);
    }
}
