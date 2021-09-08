use crate::{
    actor::{
        player::{Player, PlayerInput},
        Action, Activity, Actor,
    },
    initialization::{RunningState, TurnBasedGame},
};
use bevy_ecs::{prelude::*, schedule::ShouldRun};
use rltk::console;

pub fn handle_player_input(
    mut commands: Commands,
    input: Res<PlayerInput>,
    mut player_query: Query<(Entity, &mut Actor), With<Player>>,
)
//  -> bool
{
    if let Ok((player, mut actor)) = player_query.single_mut() {
        let action = convert_to_valid_action(input, &mut actor);
        if Action::None != action {
            commands.entity(player).insert(Activity {
                time_to_complete: 29,
                action,
            });
            // return true;
        }
    }
    // false
}

fn convert_to_valid_action(input: Res<PlayerInput>, actor: &mut Actor) -> Action {
    match input.action {
        Action::Move(direction) => {
            if direction != actor.facing && !input.is_strafing {
                Action::Face(direction)
            } else {
                println!("Player is moving: {:?}", direction);
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

pub fn is_player_waiting_for_input(player_query: Query<&Player, Without<Activity>>) -> ShouldRun {
    if let Ok(_) = player_query.single() {
        ShouldRun::Yes
    } else {
        println!("Player is busy");
        ShouldRun::No
    }
}

// pub fn pause_game(mut turn_based_game: ResMut<TurnBasedGame>) {
//     turn_based_game.state = RunningState::Paused;
// }

pub fn set_turn_based_state(In(is_running): In<bool>, mut turn_based_game: ResMut<TurnBasedGame>) {
    if is_running {
        turn_based_game.state = RunningState::Running;
        println!("Resuming game");
    } else {
        turn_based_game.state = RunningState::Paused;
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;

    use crate::{
        actor::{
            player::{systems::set_turn_based_state, Player, PlayerInput},
            Action, Activity, Actor, ActorBundle,
        },
        constants::facings::{NORTH, SOUTH},
        initialization::{RunningState, TurnBasedGame},
    };

    use super::{handle_player_input, is_input_valid};

    fn test_world() -> World {
        let mut world = World::new();
        world.insert_resource(PlayerInput {
            action: Action::None,
            ..Default::default()
        });
        world.insert_resource(TurnBasedGame::default());
        world
    }

    fn player_stage() -> SystemStage {
        SystemStage::single(
            handle_player_input
                .system()
                // .chain(set_turn_based_state.system())
                .with_run_criteria(is_input_valid.system()),
        )
    }

    fn player(world: &mut World) -> Entity {
        world
            .spawn()
            .insert_bundle(ActorBundle::default())
            .insert(Player)
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

        let facing = world.get::<Actor>(player).unwrap().facing;
        let activity = world.get::<Activity>(player).unwrap();
        let turn_based_game = world.get_resource::<TurnBasedGame>().unwrap();
        assert_eq!(SOUTH, facing);
        assert_eq!(Action::None, activity.action);
        assert_eq!(RunningState::Paused, turn_based_game.state);
    }

    #[test]
    fn move_action() {
        let mut world = test_world();
        let player = player(&mut world);
        world.get_resource_mut::<PlayerInput>().unwrap().action = Action::Move(SOUTH);

        player_stage().run(&mut world);

        let facing = world.get::<Actor>(player).unwrap().facing;
        let activity = world.get::<Activity>(player).unwrap();
        let turn_based_game = world.get_resource::<TurnBasedGame>().unwrap();
        assert_eq!(SOUTH, facing);
        assert_eq!(Action::Move(SOUTH), activity.action);
        assert_eq!(RunningState::Running, turn_based_game.state);
    }

    #[test]
    fn change_facing_action() {
        let mut world = test_world();
        let player = player(&mut world);
        world.get_resource_mut::<PlayerInput>().unwrap().action = Action::Move(NORTH);

        player_stage().run(&mut world);

        let facing = world.get::<Actor>(player).unwrap().facing;
        let activity = world.get::<Activity>(player).unwrap();
        let turn_based_game = world.get_resource::<TurnBasedGame>().unwrap();
        assert_eq!(NORTH, facing);
        assert_eq!(Action::Wait, activity.action);
        assert_eq!(RunningState::Running, turn_based_game.state);
    }

    #[test]
    fn cannot_accept_input_while_game_is_running() {
        let mut world = test_world();
        let player = player(&mut world);
        world.get_resource_mut::<TurnBasedGame>().unwrap().state = RunningState::Running;
        world.get_resource_mut::<PlayerInput>().unwrap().action = Action::Move(NORTH);

        player_stage().run(&mut world);
        let activity = world.get::<Activity>(player).unwrap();
        let turn_based_game = world.get_resource::<TurnBasedGame>().unwrap();
        assert_eq!(Action::None, activity.action);
        assert_eq!(RunningState::Running, turn_based_game.state);
    }
}
