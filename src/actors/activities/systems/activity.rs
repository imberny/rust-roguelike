use bevy_ecs::prelude::*;
use rltk::RGB;
use std::{collections::HashSet, convert::*};

use crate::{
    actors::{Action, Activity, Actor, Attack},
    core::types::{
        Cardinal, Direction, Facing, GridPos, GridPosPredicate, Int, IntoGridPos, RealPos,
    },
    core::TimeProgressionEvent,
    game_world::{AreaGrid, Viewshed},
    rendering::Renderable,
    util::algorithms::{
        field_of_view::{self, FieldOfView},
        QuadrantRow,
    },
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

    if is_blocking(result_position) {
        let facing = compute_facing(counterclockwise_slide, cardinal);
        result_position = (facing.reversed() * RealPos::from(delta) + RealPos::from(pos)).round();
        if is_blocking(result_position) {
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

    if is_blocking(result_position) {
        result_position = slide(*pos, direction, cardinal, is_blocking);
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
                        time_to_complete: 150,
                    });
                    // initiate attack, mark all tiles in range
                    // add attacking activity
                    // when attacking activity is done, perform attack on all tiles
                    // attack is changed into active effect on tile
                }
                Action::Attack => {
                    let origin = pos.clone();
                    // let facing: Facing = actor.facing.into();

                    // let positions = vec![
                    //     GridPos::new(-1, 1),
                    //     GridPos::new(0, 1),
                    //     GridPos::new(1, 1),
                    //     GridPos::new(0, 2),
                    // ];

                    let mut positions: HashSet<GridPos> = HashSet::new();

                    let fov = field_of_view::quadratic_fov(2, actor.facing.into(), 0.5, 0.0);

                    QuadrantRow::new(origin, actor.facing).scan(
                        &mut |pos| positions.insert(pos),
                        &|pos| fov.sees(pos),
                        &|_pos| false,
                    );

                    for pos in positions.iter() {
                        commands.spawn().insert(pos.clone()).insert(Renderable {
                            glyph: rltk::to_cp437('*'),
                            fg: RGB::named(rltk::ROYAL_BLUE),
                            bg: RGB::named(rltk::RED),
                        });
                    }

                    // for attack_pos in positions {
                    //     let delta = GridPos::new(0, -1);

                    //     let facing:Facing = actor.facing.into();
                    //     let mut result_facing: GridPos =
                    //         (facing.reversed() * RealPos::from(delta)).round();

                    //     let pos: GridPos = origin + ;
                    //     commands.spawn().insert(pos).insert(Renderable {
                    //         glyph: rltk::to_cp437('*'),
                    //         fg: RGB::named(rltk::ROYAL_BLUE),
                    //         bg: RGB::named(rltk::RED),
                    //     });
                    // }
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

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;

    use crate::{
        actors::{Action, Activity, ActorBundle},
        core::types::{Cardinal, Direction, Facing, GridPos, IntoGridPos, RealPos},
        game_world::{AreaGrid, TileType},
        test::activity::MoveTestCases,
        util::helpers::deserialize,
    };

    use super::{do_move, process_activities, slide};

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
        let test_cases: MoveTestCases = deserialize("src/test/data/moves.ron");

        for (direction, cardinal, x, y, is_blocked) in test_cases.cases {
            let expected = GridPos::new(x, y);
            let pos = slide(from, direction, cardinal, &|pos| {
                pos != expected || is_blocked
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

    #[test]
    fn attacking() {
        let origin = GridPos::zero();
        let facing: Facing = Cardinal::NorthEast.into();

        let positions = vec![
            // GridPos::new(-1, 1),
            // GridPos::new(0, 1),
            // GridPos::new(1, 1),
            GridPos::new(0, 2),
        ];

        let expected_positions = vec![
            // GridPos::new(0, 1),
            // GridPos::new(1, 1),
            // GridPos::new(1, 0),
            GridPos::new(2, -2),
        ];

        let rotated_positions: Vec<GridPos> = positions
            .iter()
            .map(|pos| {
                let real_pos: RealPos = pos.clone().into();
                let rotated_pos = facing.reversed() * real_pos;
                origin + rotated_pos.round()
            })
            .collect();
        assert_eq!(expected_positions, rotated_positions);
    }
}
