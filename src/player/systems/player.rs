use crate::{
    actor::{constants::MOVE_WAIT, Action, Actor},
    constants::facings,
    player::{Player, PlayerInput},
};
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
                    if facings::KEEP != direction {
                        player.facing = direction;
                    }
                    MOVE_WAIT
                } else {
                    Action::Move(direction)
                }
            }
        };
        player.action = action;
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;

    use crate::{
        actor::{Action, Actor},
        player::{Player, PlayerInput},
    };

    use super::handle_player_input;

    #[test]
    fn no_action() {
        let mut world = World::new();
        let player = world.spawn().insert_bundle((Actor::default(), Player)).id();
        world.insert_resource(PlayerInput {
            action: Action::None,
            ..Default::default()
        });
        let mut stage = SystemStage::single(handle_player_input.system());

        stage.run(&mut world);

        let actor = world.get::<Actor>(player).unwrap();

        assert_eq!(Action::None, actor.action);
    }
}
