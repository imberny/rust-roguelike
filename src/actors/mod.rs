use bevy::prelude::*;

use self::{
    activities::systems::{do_activities, progress_activities},
    effects::systems::progress_effects,
    input::PlayerInput,
    systems::{handle_player_input, is_input_valid},
};
use crate::{
    core::{InputStage, TurnGameStage},
    AppState,
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

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(PlayerInput::default());
        app.add_system_set(SystemSet::on_update(AppState::Paused).with_system(
                handle_player_input
                    // .with_run_criteria(is_input_valid.system()
                // ),
            ));
        // ecs.input.add_system_to_stage();
        app.add_system_set(
            SystemSet::on_update(AppState::Running)
                .before(ActorSystems::Action)
                .with_system(progress_activities.system())
                .with_system(progress_effects.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Running)
                .label(ActorSystems::Action)
                .with_system(do_activities.system()),
        );
        // ecs.game_logic
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum ActorSystems {
    Action,
}

// pub fn register(ecs: &mut GameRunner) {}
