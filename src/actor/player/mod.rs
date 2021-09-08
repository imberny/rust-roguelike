use crate::initialization::{is_game_running, CoreStage};
use bevy_ecs::{
    prelude::IntoChainSystem,
    schedule::{ParallelSystemDescriptorCoercion, Schedule},
    system::IntoSystem,
};

mod input;
mod player;
pub mod systems;

pub use input::*;
pub use player::Player;

use self::systems::{
    can_accept_input, handle_player_input, is_player_waiting_for_input, pause_game, resume_game,
};

pub fn register(schedule: &mut Schedule) {
    schedule
        .add_system_to_stage(
            CoreStage::Decision,
            handle_player_input
                .system()
                .chain(resume_game.system())
                .with_run_criteria(can_accept_input.system()),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            pause_game
                .system()
                .with_run_criteria(is_game_running.system())
                .with_run_criteria(is_player_waiting_for_input.system()),
        );
}
