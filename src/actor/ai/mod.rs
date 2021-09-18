use self::systems::monster_ai;
use crate::{core::TurnGameStage, game::GameRunner};
use bevy_ecs::{
    schedule::{SystemLabel, SystemSet},
    system::IntoSystem,
};

mod monster;
pub mod systems;

pub use monster::Monster;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct AISystems;

pub fn register(ecs: &mut GameRunner) {
    ecs.game_logic.add_system_set_to_stage(
        TurnGameStage::Decision,
        SystemSet::new()
            .label(AISystems)
            .with_system(monster_ai.system()),
    );
}
