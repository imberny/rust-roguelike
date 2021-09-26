use bevy_ecs::prelude::*;
use std::convert::*;

use crate::{
    actors::{Action, Activity, Actor, Attack},
    core::types::{
        Cardinal, Direction, Facing, GridPos, GridPosPredicate, Int, IntoGridPos, RealPos,
    },
    core::TimeProgressionEvent,
    game_world::{AreaGrid, Viewshed},
};

fn compute_facing(direction: Direction, cardinal: Cardinal) -> Facing {
    let direction: Facing = direction.into();
    let cardinal: Facing = cardinal.into();
    direction * cardinal
}

fn rotate_facing(cardinal: Cardinal, offset: Int) -> Cardinal {
    let cardinal_index: Int = cardinal.into();
    ((cardinal_index + offset) % 8).into()
}

fn slide(
    pos: &GridPos,
    direction: Direction,
    cardinal: Cardinal,
    is_blocking: &GridPosPredicate,
) -> GridPos {
    let delta = GridPos::new(0, -1);

    let clockwise_slide: Direction = rotate_facing(direction.into(), 1).into();
    let counterclockwise_slide: Direction = rotate_facing(direction.into(), -1).into();

    let facing = compute_facing(clockwise_slide, cardinal);
    let mut result_position: GridPos =
        (facing.reversed() * RealPos::from(delta) + RealPos::from(*pos)).as_grid_pos();

    if is_blocking(result_position) {
        let facing = compute_facing(counterclockwise_slide, cardinal);
        result_position =
            (facing.reversed() * RealPos::from(delta) + RealPos::from(*pos)).as_grid_pos();
        if is_blocking(result_position) {
            return GridPos::zero();
        }
    }
    result_position
}

fn do_move(
    pos: &mut GridPos,
    direction: Direction,
    cardinal: Cardinal,
    is_blocking: &GridPosPredicate,
) {
    let delta = GridPos::new(0, -1);

    let facing = compute_facing(direction, cardinal);
    let mut result_position: GridPos =
        (facing.reversed() * RealPos::from(delta) + RealPos::from(*pos)).as_grid_pos();

    if is_blocking(result_position) {
        result_position = slide(pos, direction, cardinal, is_blocking);
    }
    result_position.x = result_position.x.clamp(0, 79);
    result_position.y = result_position.y.clamp(0, 49);

    pos.x = result_position.x;
    pos.y = result_position.y;
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
                    do_move(&mut pos, direction, actor.facing, &|pos| {
                        map.is_blocking(pos)
                    });
                    // TODO: replace with event writer
                    viewshed.dirty = true;
                }
                Action::Face(cardinal) => {
                    actor.facing = cardinal;
                    viewshed.dirty = true;
                }
                Action::Turn(direction) => {
                    actor.facing = rotate_facing(actor.facing, direction.into());
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
        core::types::{Cardinal, Direction, GridPos},
        game_world::{AreaGrid, TileType},
    };

    use super::{process_activities, slide};

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
                action: Action::Move(Direction::Back),
                ..Default::default()
            })
            .id();

        let mut stage = SystemStage::single(process_activities.system());
        stage.run(&mut world);

        assert!(world.get::<Activity>(entity).is_none());
        let position = world.get::<GridPos>(entity).unwrap();
        assert_eq!(GridPos::new(0, 1), *position);
    }

    #[test]
    fn slide_test() {
        let from = GridPos::zero();
        let test_cases = vec![
            (
                Direction::Back,
                Cardinal::NorthWest,
                GridPos::new(0, 0),
                true,
            ),
            (
                Direction::Back,
                Cardinal::NorthWest,
                GridPos::new(0, 1),
                false,
            ),
            (
                Direction::Back,
                Cardinal::NorthWest,
                GridPos::new(1, 0),
                false,
            ),
            (
                Direction::Left,
                Cardinal::NorthWest,
                GridPos::new(-1, 0),
                false,
            ),
            (
                Direction::Left,
                Cardinal::NorthWest,
                GridPos::new(0, 1),
                false,
            ),
        ];

        for (dir, card, expected, block) in test_cases {
            let pos = slide(&from, dir, card, &|pos| pos != expected || block);

            assert_eq!(expected, pos);
        }
    }
}
