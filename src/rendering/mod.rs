use crate::initialization::CoreStage;
use bevy_ecs::{
    schedule::{Schedule, SystemSet},
    system::IntoSystem,
};

mod render;
mod renderable;
pub mod systems;

pub use render::*;
pub use renderable::Renderable;

use self::systems::apply_player_viewsheds;

pub fn register(schedule: &mut Schedule) {
    schedule.add_system_set_to_stage(
        CoreStage::Draw,
        SystemSet::new().with_system(apply_player_viewsheds.system()),
    );
}
