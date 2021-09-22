use self::activity::systems::{advance_activities, process_activities};
use crate::{core::TurnGameStage, game::GameRunner};
use bevy_ecs::{
    schedule::{SystemLabel, SystemSet},
    system::IntoSystem,
};

mod activity;
mod actor;
pub mod constants;
pub mod player;
pub mod systems;

pub use activity::*;
pub use actor::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum ActorSystems {
    Action,
}

pub fn register(ecs: &mut GameRunner) {
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
