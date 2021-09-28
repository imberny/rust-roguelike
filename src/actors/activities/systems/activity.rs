use bevy_ecs::prelude::*;
use rltk::RGB;
use std::{convert::*, f32::consts::PI};

use crate::{
    actors::{effects::Effect, Action, Activity, Actor},
    core::types::{
        Cardinal, Direction, Facing, GridPos, GridPosPredicate, Int, IntoGridPos, RealPos,
    },
    core::TimeIncrementEvent,
    game_world::{AreaGrid, Viewshed},
    rendering::Renderable,
    util::algorithms::{
        field_of_view::{self, FieldOfView},
        symmetric_shadowcasting,
    },
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
                        time_to_complete: 150,
                    });
                }
                Action::Attack => {
                    do_attack(*pos, actor.facing.into(), &mut commands, &|pos| {
                        map.is_blocking(pos)
                    });
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

fn project_angle(pos: GridPos, radius: f32, angle_radians: f32) -> GridPos {
    // let degrees_radians = angle_radians + std::f32::consts::PI;
    GridPos::new(
        -(pos.x + (0.0 - radius * f32::sin(angle_radians)).round() as i32),
        pos.y + (radius * f32::cos(angle_radians)).round() as i32,
    )
}

fn as_cardinal(pos: &GridPos) -> Cardinal {
    match pos.x.signum().cmp(&pos.y.signum()) {
        std::cmp::Ordering::Less => Cardinal::NorthWest,
        std::cmp::Ordering::Equal => {
            if pos.x.signum() < 0 {
                if pos.x < pos.y {
                    Cardinal::West
                } else {
                    Cardinal::SouthWest
                }
            } else if pos.x < pos.y {
                Cardinal::North
            } else {
                Cardinal::NorthEast
            }
        }
        std::cmp::Ordering::Greater => Cardinal::SouthEast,
    }
}

// https://math.stackexchange.com/questions/383321/rotating-x-y-points-45-degrees
fn rotate_45(pos: &GridPos) -> GridPos {
    let i = 1.0;
    let x = pos.x as f64;
    let y = pos.y as f64;

    let mut x_prime = ((x + y) / 2.0_f64.sqrt()) as Int;
    let mut y_prime = ((x - y) / 2.0_f64.sqrt()) as Int;

    x_prime += x_prime.signum();
    y_prime += y_prime.signum();

    if x_prime.abs() == y_prime.abs() {
        x_prime = x_prime.signum() * pos.y.abs();
        y_prime = y_prime.signum() * pos.y.abs();
    }

    if x_prime == 0 && y_prime == 0 {
        x_prime = 1;
        y_prime = -1;
    }

    GridPos::new(x_prime, y_prime)
}

fn rotate(pos: &GridPos) -> GridPos {
    // if diagonal:
    //  (y, x)
    // else:
    //  (x + y, y)
    if pos.x.signum() == pos.y.signum() {
        // GridPos::new(-pos.y, -pos.x)
        GridPos::new(-(pos.y), -(pos.y - pos.x))
    } else {
        GridPos::new(-(pos.x + pos.y), -pos.y)
    }
}

fn do_attack(
    origin: GridPos,
    facing: Facing,
    commands: &mut Commands,
    is_blocking: &GridPosPredicate,
) {
    // let fov = field_of_view::quadratic_fov(4, facing, 1.2, 0.8);
    // let pattern: Vec<GridPos> = vec![
    //     GridPos::new(0, 1),
    //     GridPos::new(0, 2),
    //     GridPos::new(0, 3),
    //     GridPos::new(-1, 3),
    //     GridPos::new(1, 3),
    //     GridPos::new(0, 4),
    //     GridPos::new(-1, 4),
    //     GridPos::new(1, 4),
    // ];
    let pattern: Vec<GridPos> = vec![
        GridPos::new(0, 1),
        GridPos::new(0, 2),
        GridPos::new(0, 3),
        GridPos::new(-1, 3),
        // GridPos::new(-2, 3),
        GridPos::new(1, 3),
        // GridPos::new(2, 3),
        GridPos::new(0, 4),
        GridPos::new(-1, 4),
        // GridPos::new(-3, 4),
        GridPos::new(1, 4),
        // GridPos::new(3, 4),
    ];
    // let fov = field_of_view::pattern_fov(pattern, facing);
    // let fov = field_of_view::cone_fov(3, PI / 8.0, facing);
    // let positions = symmetric_shadowcasting(origin, &|pos| fov.sees(pos), is_blocking);
    // for pos in positions.iter() {
    for pos in pattern.iter() {
        // let card = as_cardinal(pos);

        // let radius = std::cmp::max(pos.x, pos.y) as f32;
        // let target = project_angle(*pos, radius, PI / 4.0);
        let target = if facing == Cardinal::North.into() {
            GridPos::new(-pos.x, -pos.y)
        } else {
            // rotate_45(pos)
            rotate(pos)
        };
        // let target = pos.clone();
        commands
            .spawn()
            // .insert(pos.clone())
            .insert(target + origin)
            .insert(Renderable {
                glyph: rltk::to_cp437('*'),
                fg: RGB::named(rltk::ROYAL_BLUE),
                bg: RGB::named(rltk::RED),
            })
            .insert(Effect { time_left: 20 });
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
