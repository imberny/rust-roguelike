use self::systems::{handle_player_input, is_input_valid};
use crate::{core::InputStage, game::GameRunner};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::IntoSystem};

mod input;
mod player;
pub mod systems;

pub use input::*;
pub use player::Player;

pub fn register(ecs: &mut GameRunner) {
    ecs.world.insert_resource(PlayerInput::default());
    ecs.input.add_system_to_stage(
        InputStage::Handle,
        handle_player_input
            .system()
            .with_run_criteria(is_input_valid.system()),
    );

    // .add_system_to_stage(
    //     CoreStage::First,
    //     pause_game
    //         .system()
    //         .with_run_criteria(is_game_running.system())
    //         .with_run_criteria(is_player_waiting_for_input.system()),
    // );
}
