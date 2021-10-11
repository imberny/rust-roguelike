use bevy::prelude::*;
use std::convert::*;

use crate::{
    actors::{effects::Effect, Action, Activity, Actor, Weapon},
    core::types::{Cardinal, Direction, Facing, Int, Predicate},
    core::{
        types::{GridPos, Increment},
        TimeIncrementEvent,
    },
    util::{algorithms::geometry::chessboard_rotate_and_place, helpers::GridRotator},
    world::{Renderable, Viewshed, WorldMap},
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
    world_map: Res<WorldMap>,
    mut actors: Query<
        (
            Entity,
            &Children,
            &mut Actor,
            &mut GridPos,
            &mut Viewshed,
            &Activity,
        ),
        Without<Weapon>,
    >,
    mut weapons: Query<&mut GridPos, With<Weapon>>,
) {
    for (entity, children, mut actor, mut pos, mut viewshed, activity) in actors.iter_mut() {
        if activity.time_to_complete == 0 {
            let mut new_activity: Option<Activity> = None;

            match activity.action {
                Action::Move(direction) => {
                    let area = &world_map.get_area_from_pos(&pos.0).unwrap().1;
                    pos.0 = do_move(&pos.0, direction, actor.facing, &|pos| {
                        area.is_blocking(pos)
                    });
                    let mut weapon_pos = weapons.get_mut(*children.get(0).unwrap()).unwrap();
                    weapon_pos.0 = compute_next_position(Direction::Forward, actor.facing, &pos.0);

                    // TODO: replace with event writer
                    viewshed.dirty = true;
                }
                Action::Turn(direction) => {
                    actor.facing = rotate_facing(actor.facing, direction.into());

                    let mut weapon_pos = weapons.get_mut(*children.get(0).unwrap()).unwrap();
                    weapon_pos.0 = compute_next_position(Direction::Forward, actor.facing, &pos.0);

                    viewshed.dirty = true;
                }
                Action::InitiateAttack => {
                    new_activity = Some(Activity {
                        action: Action::Attack,
                        time_to_complete: 60,
                    });
                    telegraph_attack(&pos.0, actor.facing, &mut commands);
                }
                Action::Attack => {
                    do_attack(&pos.0, actor.facing, &mut commands);
                }
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
    pos: IVec2,
    direction: Direction,
    cardinal: Cardinal,
    is_blocking: &Predicate<IVec2>,
) -> IVec2 {
    let delta = IVec2::new(0, -1);

    let clockwise_slide: Direction = rotate_facing(direction.into(), 1).into();
    let counterclockwise_slide: Direction = rotate_facing(direction.into(), -1).into();

    let facing = compute_facing(clockwise_slide, cardinal);
    let mut result_position: IVec2 = facing.inverse().rot_i(&delta) + pos;

    if is_blocking(&result_position) {
        let facing = compute_facing(counterclockwise_slide, cardinal);
        result_position = facing.inverse().rot_i(&delta) + pos;
        if is_blocking(&result_position) {
            return pos;
        }
    }
    result_position
}

fn do_move(
    pos: &IVec2,
    direction: Direction,
    cardinal: Cardinal,
    is_blocking: &Predicate<IVec2>,
) -> IVec2 {
    let mut next_position = compute_next_position(direction, cardinal, pos);

    if is_blocking(&next_position) {
        next_position = slide(*pos, direction, cardinal, is_blocking);
    }
    next_position.x = next_position.x.clamp(0, 79);
    next_position.y = next_position.y.clamp(0, 49);

    next_position
}

fn compute_next_position(direction: Direction, cardinal: Cardinal, pos: &IVec2) -> IVec2 {
    let delta = IVec2::new(0, -1);
    let facing = compute_facing(direction, cardinal);
    facing.inverse().rot_i(&delta) + *pos
}

fn telegraph_attack(origin: &IVec2, facing: Cardinal, commands: &mut Commands) {
    let pattern: Vec<IVec2> = vec![
        IVec2::new(0, -1),
        IVec2::new(0, -2),
        IVec2::new(0, -3),
        IVec2::new(-1, -3),
        IVec2::new(-2, -3),
        IVec2::new(1, -3),
        IVec2::new(2, -3),
        IVec2::new(-1, -4),
    ];

    let positions: Vec<IVec2> = chessboard_rotate_and_place(origin, &pattern, facing.into());
    let marker = Marker {
        time_left: 60,
        renderable: Renderable {
            glyph: '!',
            fg: Color::RED,
            bg: Color::ANTIQUE_WHITE,
        },
    };
    place_markers(&positions, marker, commands);
}

fn do_attack(origin: &IVec2, cardinal: Cardinal, commands: &mut Commands) {
    let pattern: Vec<IVec2> = vec![
        IVec2::new(0, -1),
        IVec2::new(0, -2),
        IVec2::new(0, -3),
        IVec2::new(-1, -3),
        IVec2::new(-2, -3),
        IVec2::new(1, -3),
        IVec2::new(2, -3),
        IVec2::new(-1, -4),
    ];

    let positions: Vec<IVec2> = chessboard_rotate_and_place(origin, &pattern, cardinal.into());
    let marker = Marker {
        time_left: 30,
        renderable: Renderable {
            glyph: '*',
            fg: Color::NAVY,
            bg: Color::RED,
        },
    };
    place_markers(&positions, marker, commands);
}

#[derive(Debug, Clone, Copy)]
struct Marker {
    time_left: Increment,
    renderable: Renderable,
}

fn place_markers(positions: &[IVec2], marker: Marker, commands: &mut Commands) {
    positions.iter().for_each(|pos| {
        commands
            .spawn()
            .insert(GridPos(*pos))
            .insert(marker.renderable)
            .insert(Effect {
                time_left: marker.time_left,
            });
    });
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{
        actors::{Action, Activity, ActorBundle},
        core::types::{Direction, GridPos},
        test,
        world::{AreaGrid, TileType},
    };

    use super::{do_activities, slide};

    fn test_map() -> AreaGrid {
        AreaGrid {
            tiles: vec![TileType::Floor; 80 * 50],
            width: 80,
            height: 50,
            revealed: vec![false; 80 * 50],
            visible: vec![false; 80 * 50],
            ..Default::default()
        }
    }

    #[test]
    fn consume_activity_upon_completion() {
        let mut world = World::new();
        world.spawn().insert(test_map());
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
        world.spawn().insert(test_map());
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

        let activity = world.get::<Activity>(entity);
        assert!(activity.is_none());
        let position = world.get::<GridPos>(entity).unwrap();
        assert_eq!(IVec2::new(0, 1), position.0);
    }

    #[test]
    fn slide_test() {
        let from = IVec2::ZERO;

        for case in test::activity::cases() {
            let (x, y) = case.expected;
            let expected = IVec2::new(x, y);
            let pos = slide(from, case.direction, case.cardinal, &|pos| {
                *pos != expected || case.is_blocked
            });

            assert_eq!(expected, pos);
        }
    }
}
