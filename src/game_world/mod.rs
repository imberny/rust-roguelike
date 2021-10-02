mod map;
pub use map::{AreaGrid, TileType};

mod viewshed;
pub use viewshed::Viewshed;

pub mod systems;

use bevy::prelude::*;

use crate::{
    core::{RenderingStage, TurnGameStage},
    AppState,
};

use self::systems::{apply_player_viewsheds, update_viewsheds};

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_exit(AppState::Running)
                .label(MapSystems::Viewshed)
                .with_system(update_viewsheds),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Running)
                .with_system(apply_player_viewsheds.after(MapSystems::Viewshed)),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum MapSystems {
    Viewshed,
}

// pub fn register(ecs: &mut GameRunner) {
//     ecs.game_logic.add_system_set_to_stage(
//         TurnGameStage::PostUpdate,
//         SystemSet::new()
//             .label(MapSystems::Viewshed)
//             .with_system(update_viewsheds.system()),
//     );
//     ecs.rendering.add_system_set_to_stage(
//         RenderingStage::Draw,
//         SystemSet::new().with_system(apply_player_viewsheds.system()),
//     );
// }
