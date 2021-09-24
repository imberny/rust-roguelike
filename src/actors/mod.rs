use self::{
    activities::systems::{advance_activities, process_activities},
    input::PlayerInput,
    systems::{handle_player_input, is_input_valid},
};
use crate::{
    core::{InputStage, TurnGameStage},
    game::GameRunner,
};
use bevy_ecs::{
    schedule::ParallelSystemDescriptorCoercion,
    schedule::{SystemLabel, SystemSet},
    system::IntoSystem,
};

mod activities;
pub use activities::*;

mod actor;
pub use actor::*;

mod player;
pub use player::Player;

pub mod constants;

pub mod input;

pub mod systems;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum ActorSystems {
    Action,
}

pub fn register(ecs: &mut GameRunner) {
    ecs.world.insert_resource(PlayerInput::default());
    ecs.input.add_system_to_stage(
        InputStage::Handle,
        handle_player_input
            .system()
            .with_run_criteria(is_input_valid.system()),
    );
    ecs.game_logic
        .add_system_set_to_stage(
            TurnGameStage::Update,
            SystemSet::new()
                .before(ActorSystems::Action)
                .with_system(advance_activities.system()),
        )
        .add_system_set_to_stage(
            TurnGameStage::Update,
            SystemSet::new()
                .label(ActorSystems::Action)
                .with_system(process_activities.system()),
        );
}
