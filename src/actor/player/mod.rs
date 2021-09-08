use crate::initialization::CoreStage;
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
    handle_player_input, is_input_valid, is_player_waiting_for_input, set_turn_based_state,
};

// pub fn register(schedule: &mut Schedule) {
//     schedule.add_system_to_stage(
//         CoreStage::Decision,
//         handle_player_input
//             .system()
//             .chain(set_turn_based_state.system())
//             .with_run_criteria(is_input_valid.system())
//             .with_run_criteria(is_player_waiting_for_input.system()),
//     );
//     // .add_system_to_stage(
//     //     CoreStage::First,
//     //     pause_game
//     //         .system()
//     //         .with_run_criteria(is_game_running.system())
//     //         .with_run_criteria(is_player_waiting_for_input.system()),
//     // );
// }
