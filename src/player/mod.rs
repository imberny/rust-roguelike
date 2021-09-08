use crate::initialization::{resume_game, CoreStage};
use bevy_ecs::{
    schedule::{Schedule, SystemLabel, SystemSet},
    system::IntoSystem,
};

mod input;
mod player;
pub mod systems;

pub use input::*;
pub use player::Player;

use self::systems::{can_accept_input, handle_player_input, is_player_busy};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct PlayerInputHandler;

pub fn register(schedule: &mut Schedule) {
    schedule
        .add_system_set_to_stage(
            CoreStage::Decision,
            SystemSet::new()
                .label(PlayerInputHandler)
                .with_run_criteria(can_accept_input.system())
                .with_system(handle_player_input.system()),
        )
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .after(PlayerInputHandler)
                .with_run_criteria(is_player_busy.system())
                .with_system(resume_game.system()),
        );
}
