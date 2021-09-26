use bevy_ecs::prelude::*;

use crate::{
    actors::{Action, Activity, Actor, Attack},
    core::types::{Facing, GridPos, IntoGridPos, RealPos},
    core::TimeProgressionEvent,
    game_world::{AreaGrid, Viewshed},
};

fn do_move(pos: &mut GridPos, direction: Facing, facing: Facing, map: &Res<AreaGrid>) {
    let delta = GridPos::new(0, -1);

    // let result_direction =
    let mut result_position: GridPos = ((direction * facing).reversed() * RealPos::from(delta)
        + RealPos::from(*pos))
    .as_grid_pos();
    result_position.x = result_position.x.clamp(0, 79);
    result_position.y = result_position.y.clamp(0, 49);

    if !map.is_blocking(result_position) {
        pos.x = result_position.x;
        pos.y = result_position.y;
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
    mut actors: Query<(Entity, &mut Actor, &mut GridPos, &mut Viewshed, &Activity)>,
) {
    for (entity, mut actor, mut pos, mut viewshed, activity) in actors.iter_mut() {
        if activity.time_to_complete == 0 {
            // console::log("Doing something");
            match activity.action {
                Action::Move(direction) => {
                    do_move(&mut pos, direction, actor.facing, &map);
                    // TODO: replace with event writer
                    viewshed.dirty = true;
                }
                Action::Face(direction) => {
                    actor.facing = actor.facing * direction;
                    viewshed.dirty = true;
                }
                Action::Attack => {}
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
        actors::{Action, Activity, ActorBundle},
        core::{constants::SOUTH, types::GridPos},
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
                action: Action::Move(SOUTH),
                ..Default::default()
            })
            .id();

        let mut stage = SystemStage::single(process_activities.system());
        stage.run(&mut world);

        assert!(world.get::<Activity>(entity).is_none());
        let position = world.get::<GridPos>(entity).unwrap();
        assert_eq!(GridPos::new(0, 1), *position);
    }
}
