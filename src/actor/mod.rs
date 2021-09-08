use self::systems::{process_move_actions, update_viewsheds};
use crate::initialization::CoreStage;
use bevy_ecs::{
    schedule::{Schedule, SystemLabel, SystemSet},
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

pub fn register(game_logic: &mut Schedule) {
    game_logic
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .label(ActorSystems::Action)
                .with_system(process_move_actions.system()),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .label(ActorSystems::Viewshed)
                .with_system(update_viewsheds.system()),
        );
    ai::register(game_logic);
}
