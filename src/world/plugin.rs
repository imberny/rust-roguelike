use bevy::prelude::*;

use crate::{AppState, SystemLabels};

use super::{
    generator::generate_map_system,
    systems::{apply_player_viewsheds, update_renderables, update_viewsheds},
    WorldMap,
};

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>()
            .add_startup_system(generate_map_system.label(SystemLabels::Generation))
            .add_system_set(
                SystemSet::on_exit(AppState::Running)
                    .label(MapSystems::Viewshed)
                    .with_system(update_viewsheds),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Running)
                    .with_system(apply_player_viewsheds.after(MapSystems::Viewshed)),
            )
            .add_system_set(SystemSet::on_exit(AppState::Running).with_system(update_renderables));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum MapSystems {
    Viewshed,
}
