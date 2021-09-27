use self::{
    activities::systems::{do_activities, progress_activities},
    effects::systems::progress_effects,
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

pub mod effects;

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
                .with_system(progress_activities.system())
                .with_system(progress_effects.system()),
        )
        .add_system_set_to_stage(
            TurnGameStage::Update,
            SystemSet::new()
                .label(ActorSystems::Action)
                .with_system(do_activities.system()),
        );
}
