use self::systems::{advance_activities, process_activities, update_viewsheds};
use crate::{core::TurnGameStage, game::GameRunner};
use bevy_ecs::{
    schedule::{SystemLabel, SystemSet},
    system::IntoSystem,
};

mod action;
mod activity;
mod actor;
pub mod ai;
pub mod constants;
pub mod player;
pub mod systems;
mod viewshed;

pub use action::Action;
pub use activity::Activity;
pub use actor::*;
pub use viewshed::Viewshed;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum ActorSystems {
    Action,
    Viewshed,
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
        )
        .add_system_set_to_stage(
            TurnGameStage::PostUpdate,
            SystemSet::new()
                .label(ActorSystems::Viewshed)
                .with_system(update_viewsheds.system()),
        );
    ai::register(ecs);
}
