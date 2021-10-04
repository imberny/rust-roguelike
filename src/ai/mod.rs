use bevy::prelude::*;

use self::systems::monster_ai;

pub mod systems;

mod monster;
pub use monster::Monster;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label(AISystems)
                .with_system(monster_ai.system()),
        );
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct AISystems;
