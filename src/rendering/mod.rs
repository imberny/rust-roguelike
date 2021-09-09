use crate::{core::RenderingStage, game::ECS};
use bevy_ecs::{schedule::SystemSet, system::IntoSystem};

mod render;
mod renderable;
pub mod systems;

pub use render::*;
pub use renderable::Renderable;

use self::systems::apply_player_viewsheds;

pub fn register(ecs: &mut ECS) {
    ecs.rendering.add_system_set_to_stage(
        RenderingStage::Draw,
        SystemSet::new().with_system(apply_player_viewsheds.system()),
    );
}
