use crate::{
    actor::{
        constants::MOVE_WAIT,
        player::{Player, PlayerInput},
        Action, Activity, Actor,
    },
    constants::facings,
    initialization::{RunningState, TurnBasedGame},
};
use bevy_ecs::{prelude::*, schedule::ShouldRun};

pub fn handle_player_input(
    input: Res<PlayerInput>,
    mut player_query: Query<(&mut Actor, &mut Activity), With<Player>>,
) -> bool {
    if let Ok((mut player, mut activity)) = player_query.single_mut() {
        convert_to_valid_action(input, &mut player, &mut activity);
        return Action::None != activity.action;
    }
    false
}

fn convert_to_valid_action(
    input: Res<PlayerInput>,
    mut player: &mut Actor,
    mut activity: &mut Activity,
) {
    let action = match input.action {
        Action::None => Action::None,
        Action::Move(direction) => {
            if direction != player.facing && !input.is_strafing {
                if facings::KEEP != direction {
                    player.facing = direction;
                }
                MOVE_WAIT
            } else {
                println!("Player is moving: {:?}", direction);
                Action::Move(direction)
            }
        }
    };
    activity.action = action;
}

pub fn can_accept_input(
    turn_based_state: Res<TurnBasedGame>,
    input: Res<PlayerInput>,
) -> ShouldRun {
    match turn_based_state.state {
        RunningState::Paused => {
            if input.is_valid() {
                ShouldRun::Yes
            } else {
                ShouldRun::No
            }
        }
        _ => ShouldRun::No,
    }
}

pub fn is_player_waiting_for_input(player_query: Query<&Activity, With<Player>>) -> ShouldRun {
    if let Ok(activity) = player_query.single() {
        match activity.action {
            Action::None => ShouldRun::Yes,
            _ => ShouldRun::No,
        }
    } else {
        ShouldRun::No
    }
}

pub fn is_player_busy(player_query: Query<&Activity, With<Player>>) -> ShouldRun {
    if let Ok(activity) = player_query.single() {
        match activity.action {
            Action::None => ShouldRun::No,
            _ => {
                println!("Player is busy");
                return ShouldRun::Yes;
            }
        }
    } else {
        ShouldRun::No
    }
}

pub fn pause_game(mut turn_based_game: ResMut<TurnBasedGame>) {
    turn_based_game.state = RunningState::Paused;
}

pub fn resume_game(In(can_resume): In<bool>, mut turn_based_game: ResMut<TurnBasedGame>) {
    if can_resume {
        turn_based_game.state = RunningState::Running;
        println!("Resuming game");
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;

    use crate::{
        actor::{
            player::{systems::resume_game, Player, PlayerInput},
            Action, Activity, ActorBundle,
        },
        initialization::TurnBasedGame,
    };

    use super::handle_player_input;

    #[test]
    fn no_action() {
        let mut world = World::new();
        let player = world
            .spawn()
            .insert_bundle(ActorBundle::default())
            .insert(Player)
            .id();
        world.insert_resource(PlayerInput {
            action: Action::None,
            ..Default::default()
        });
        world.insert_resource(TurnBasedGame::default());
        let mut stage =
            SystemStage::single(handle_player_input.system().chain(resume_game.system()));

        stage.run(&mut world);

        let activity = world.get::<Activity>(player).unwrap();

        assert_eq!(Action::None, activity.action);
    }
}
