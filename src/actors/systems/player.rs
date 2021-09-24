use crate::actors::{input::PlayerInput, Action, Activity, Actor, Player};
use bevy_ecs::{prelude::*, schedule::ShouldRun};

pub fn handle_player_input(
    mut commands: Commands,
    input: Res<PlayerInput>,
    mut player_query: Query<(Entity, &mut Actor), With<Player>>,
) {
    if let Ok((player, mut actor)) = player_query.single_mut() {
        let action = convert_to_valid_action(input, &mut actor);
        if Action::None != action {
            commands.entity(player).insert(Activity {
                time_to_complete: 29,
                action,
            });
        }
    }
}

fn convert_to_valid_action(input: Res<PlayerInput>, actor: &mut Actor) -> Action {
    match input.action {
        Action::Move(direction) => {
            if direction != actor.facing && !input.is_strafing {
                Action::Face(direction)
            } else {
                // console::log(format!("Player is moving: {:?}", direction));
                Action::Move(direction)
            }
        }
        _ => input.action,
    }
}

pub fn is_input_valid(input: Res<PlayerInput>) -> ShouldRun {
    if input.is_valid() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn is_player_busy(player: Query<&Player, With<Activity>>) -> ShouldRun {
    match player.single() {
        Ok(_) => ShouldRun::YesAndCheckAgain,
        Err(_) => ShouldRun::No,
    }
}

pub fn is_player_waiting_for_input(player: Query<&Player, With<Activity>>) -> ShouldRun {
    match player.single() {
        Ok(_) => ShouldRun::No,
        Err(_) => ShouldRun::Yes,
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;

    use crate::{
        actors::{input::PlayerInput, Action, Activity, ActorBundle, Player},
        core::constants::*,
    };

    use super::{handle_player_input, is_input_valid};

    fn test_world() -> World {
        let mut world = World::new();
        world.insert_resource(PlayerInput {
            action: Action::None,
            ..Default::default()
        });
        world
    }

    fn player_stage() -> SystemStage {
        SystemStage::single(
            handle_player_input
                .system()
                .with_run_criteria(is_input_valid.system()),
        )
    }

    fn player(world: &mut World) -> Entity {
        world
            .spawn()
            .insert(Player)
            .insert_bundle(ActorBundle::default())
            .id()
    }

    // fn run_system(world: &mut World, system: impl Into<SystemDescriptor>) {
    //     let mut schedule = Schedule::default();
    //     let mut update = SystemStage::parallel();
    //     update.add_system(system);
    //     schedule.add_stage("update", update);
    //     schedule.run(world);
    // }

    #[test]
    fn no_action() {
        let mut world = test_world();
        let player = player(&mut world);

        player_stage().run(&mut world);

        assert!(world.get::<Activity>(player).is_none());
    }

    #[test]
    fn input_inserts_activity() {
        let mut world = test_world();
        let player = player(&mut world);
        world.get_resource_mut::<PlayerInput>().unwrap().action = Action::Move(SOUTH);

        player_stage().run(&mut world);

        let activity = world.get::<Activity>(player).unwrap();
        assert_eq!(Action::Move(SOUTH), activity.action);
    }

    #[test]
    fn change_facing_action() {
        let mut world = test_world();
        let player = player(&mut world);
        world.get_resource_mut::<PlayerInput>().unwrap().action = Action::Move(NORTH);

        player_stage().run(&mut world);

        let activity = world.get::<Activity>(player).unwrap();
        assert_eq!(Action::Face(NORTH), activity.action);
    }
}
