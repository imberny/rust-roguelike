mod map;
mod viewshed;

pub mod systems;

use bevy_ecs::prelude::*;
pub use map::{AreaGrid, TileType};
pub use viewshed::Viewshed;

use crate::{core::RenderingStage, game::GameRunner};

use self::systems::apply_player_viewsheds;

pub fn register(ecs: &mut GameRunner) {
    ecs.rendering.add_system_set_to_stage(
        RenderingStage::Draw,
        SystemSet::new().with_system(apply_player_viewsheds.system()),
    );
}
