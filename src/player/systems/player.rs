use crate::{
    actor::{constants::MOVE_WAIT, Action, Activity, Actor},
    constants::facings,
    initialization::{RunningState, TurnBasedGame},
    player::{Player, PlayerInput},
};
use bevy_ecs::{prelude::*, schedule::ShouldRun};

pub fn handle_player_input(
    input: Res<PlayerInput>,
    mut player_query: Query<(&mut Actor, &mut Activity), With<Player>>,
) {
    if let Ok((mut player, mut activity)) = player_query.single_mut() {
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

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;

    use crate::{
        actor::{Action, Activity, ActorBundle},
        player::{Player, PlayerInput},
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
        let mut stage = SystemStage::single(handle_player_input.system());

        stage.run(&mut world);

        let activity = world.get::<Activity>(player).unwrap();

        assert_eq!(Action::None, activity.action);
    }
}
