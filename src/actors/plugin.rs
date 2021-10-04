use bevy::prelude::*;

use super::{
    activities::systems::{do_activities, progress_activities},
    effects::systems::progress_effects,
    systems::handle_player_input,
};
use crate::AppState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ActorSystems {
    Action,
}

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Paused).with_system(handle_player_input));
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
    }
}
