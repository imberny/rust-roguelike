use self::systems::monster_ai;
use crate::initialization::CoreStage;
use bevy_ecs::{
    schedule::{Schedule, SystemLabel, SystemSet},
    system::IntoSystem,
};

mod monster;
pub mod systems;

pub use monster::Monster;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct AISystems;

pub fn register(schedule: &mut Schedule) {
    schedule.add_system_set_to_stage(
        CoreStage::Decision,
        SystemSet::new()
            .label(AISystems)
            .with_system(monster_ai.system()),
    );
}
