use crate::{Player, PlayerInput, actor::{Actor, action::Action, constants::MOVE_WAIT}};
use bevy_ecs::prelude::*;

pub fn handle_player_input(
    input: Res<PlayerInput>,
    mut player_query: Query<&mut Actor, With<Player>>,
) {
    for mut player in player_query.iter_mut() {
        let action = match input.action {
            Action::None => Action::None,
            Action::Move(direction) => {
                if direction != player.facing && !input.is_strafing {
                    player.facing = direction;
                    MOVE_WAIT
                } else {
                    Action::Move(direction)
                }
            }
        };
        player.action = action;
    }
}
