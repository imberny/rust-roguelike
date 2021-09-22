mod field_of_view;
mod map;
mod quadrant;
mod viewshed;

pub mod systems;

use bevy_ecs::prelude::*;
pub use map::{AreaGrid, TileType};
pub use viewshed::Viewshed;

use crate::{
    core::{RenderingStage, TurnGameStage},
    game::GameRunner,
};

use self::systems::{apply_player_viewsheds, update_viewsheds};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum MapSystems {
    Viewshed,
}

pub fn register(ecs: &mut GameRunner) {
    ecs.rendering
        .add_system_set_to_stage(
            RenderingStage::Draw,
            SystemSet::new().with_system(apply_player_viewsheds.system()),
        )
        .add_system_set_to_stage(
            TurnGameStage::PostUpdate,
            SystemSet::new()
                .label(MapSystems::Viewshed)
                .with_system(update_viewsheds.system()),
        );
}
