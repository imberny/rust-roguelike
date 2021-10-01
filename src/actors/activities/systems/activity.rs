use bevy_ecs::prelude::*;
use rltk::RGB;
use std::convert::*;

use crate::{
    actors::{effects::Effect, Action, Activity, Actor},
    core::types::{
        Cardinal, Direction, Facing, GridPos, GridPosPredicate, Int, IntoGridPos, RealPos,
    },
    core::{types::Increment, TimeIncrementEvent},
    game_world::{AreaGrid, Viewshed},
    rendering::Renderable,
    util::algorithms::transform::chessboard_rotate_vec,
};

pub fn progress_activities(
    mut time_events: EventReader<TimeIncrementEvent>,
    mut activities: Query<&mut Activity>,
) {
    for time_event in time_events.iter() {
        for mut activity in activities.iter_mut() {
            if time_event.delta_time > activity.time_to_complete {
                activity.time_to_complete = 0;
            } else {
                activity.time_to_complete -= time_event.delta_time;
            }
        }
    }
}

pub fn do_activities(
    mut commands: Commands,
    map: Res<AreaGrid>,
    mut actors: Query<(Entity, &mut Actor, &mut GridPos, &mut Viewshed, &Activity)>,
) {
    for (entity, mut actor, mut pos, mut viewshed, activity) in actors.iter_mut() {
        if activity.time_to_complete == 0 {
            // console::log("Doing something");
            let mut new_activity: Option<Activity> = None;

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
                Action::InitiateAttack => {
                    new_activity = Some(Activity {
                        action: Action::Attack,
                        time_to_complete: 60,
                    });
                    telegraph_attack(*pos, actor.facing, &mut commands);
                }
                Action::Attack => {
                    do_attack(*pos, actor.facing, &mut commands);
                }
                // Action::Say(message) => match message.kind {
                //     MessageType::Insult => console::log("*!!$%$#&^%@"),
                //     MessageType::Threaten => console::log("Shouldn't have come here"),
                //     MessageType::Compliment => console::log("Lookin' good today!"),
                // },
                _ => (),
            }
            commands.entity(entity).remove::<Activity>();
            if let Some(activity) = new_activity {
                commands.entity(entity).insert(activity);
            }
        }
    }
}

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
    pos: GridPos,
    direction: Direction,
    cardinal: Cardinal,
    is_blocking: &GridPosPredicate,
) -> GridPos {
    let delta = GridPos::new(0, -1);

    let clockwise_slide: Direction = rotate_facing(direction.into(), 1).into();
    let counterclockwise_slide: Direction = rotate_facing(direction.into(), -1).into();

    let facing = compute_facing(clockwise_slide, cardinal);
    let mut result_position: GridPos =
        (facing.reversed() * RealPos::from(delta) + RealPos::from(pos)).round();

    if is_blocking(&result_position) {
        let facing = compute_facing(counterclockwise_slide, cardinal);
        result_position = (facing.reversed() * RealPos::from(delta) + RealPos::from(pos)).round();
        if is_blocking(&result_position) {
            return pos;
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
        (facing.reversed() * RealPos::from(delta) + RealPos::from(*pos)).round();

    if is_blocking(&result_position) {
        result_position = slide(*pos, direction, cardinal, is_blocking);
    }
    result_position.x = result_position.x.clamp(0, 79);
    result_position.y = result_position.y.clamp(0, 49);

    pos.x = result_position.x;
    pos.y = result_position.y;
}

fn telegraph_attack(origin: GridPos, facing: Cardinal, commands: &mut Commands) {
    let pattern: Vec<GridPos> = vec![
        GridPos::new(0, -1),
        GridPos::new(0, -2),
        GridPos::new(0, -3),
        GridPos::new(-1, -3),
        GridPos::new(-2, -3),
        GridPos::new(1, -3),
        GridPos::new(2, -3),
        GridPos::new(-1, -4),
    ];

    let positions = chessboard_rotate_vec(pattern, facing.into())
        .iter()
        .map(|pos| origin + *pos)
        .collect();
    let marker = Marker {
        time_left: 60,
        renderable: Renderable {
            glyph: rltk::to_cp437('!'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::ANTIQUE_WHITE),
        },
    };
    place_markers(positions, marker, commands);
}

fn do_attack(origin: GridPos, facing: Cardinal, commands: &mut Commands) {
    let pattern: Vec<GridPos> = vec![
        GridPos::new(0, -1),
        GridPos::new(0, -2),
        GridPos::new(0, -3),
        GridPos::new(-1, -3),
        GridPos::new(-2, -3),
        GridPos::new(1, -3),
        GridPos::new(2, -3),
        GridPos::new(-1, -4),
    ];

    let positions = chessboard_rotate_vec(pattern, facing.into())
        .iter()
        .map(|pos| origin + *pos)
        .collect();
    let marker = Marker {
        time_left: 30,
        renderable: Renderable {
            glyph: rltk::to_cp437('*'),
            fg: RGB::named(rltk::ROYAL_BLUE),
            bg: RGB::named(rltk::RED),
        },
    };
    place_markers(positions, marker, commands);
}

#[derive(Debug, Clone, Copy)]
struct Marker {
    time_left: Increment,
    renderable: Renderable,
}

fn place_markers(positions: Vec<GridPos>, marker: Marker, commands: &mut Commands) {
    positions.iter().for_each(|pos| {
        commands
            .spawn()
            .insert(*pos)
            .insert(marker.renderable)
            .insert(Effect {
                time_left: marker.time_left,
            });
    });
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;

    use crate::{
        actors::{Action, Activity, ActorBundle},
        core::types::{Cardinal, Direction, GridPos},
        game_world::{AreaGrid, TileType},
        test,
    };

    use super::{do_activities, do_move, slide};

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

        let mut stage = SystemStage::single(do_activities.system());
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

        let mut stage = SystemStage::single(do_activities.system());
        stage.run(&mut world);

        assert!(world.get::<Activity>(entity).is_none());
        let position = world.get::<GridPos>(entity).unwrap();
        assert_eq!(GridPos::new(0, 1), *position);
    }

    #[test]
    fn slide_test() {
        let from = GridPos::zero();

        for case in test::activity::cases() {
            let (x, y) = case.expected;
            let expected = GridPos::new(x, y);
            let pos = slide(from, case.direction, case.cardinal, &|pos| {
                *pos != expected || case.is_blocked
            });

            assert_eq!(expected, pos);
        }
    }

    #[test]
    fn move_zero() {
        do_move(
            &mut GridPos::zero(),
            Direction::Forward,
            Cardinal::North,
            &|_pos| false,
        );
    }
}
