use crate::types::{Facing, Percentage, Position};
use bevy_ecs::bundle::Bundle;

use super::{Activity, Viewshed};

const BASE_HEALTH: i32 = 100;
const BASE_SPEED: i32 = 100;

#[derive(Debug)]
pub struct CharacterSheet {
    pub health: i32, // maybe derive health from attributes
    pub speed: i32,
    // attributes...
}

impl Default for CharacterSheet {
    fn default() -> Self {
        Self {
            health: BASE_HEALTH,
            speed: BASE_SPEED,
        }
    }
}

#[derive(Debug, Default)]
pub struct CharacterProperties {
    pub health: Percentage,
    pub energy: i32, // Available time units
}

#[derive(Debug, Default)]
pub struct Actor {
    pub sheet: CharacterSheet,
    pub properties: CharacterProperties,
    pub facing: Facing,
}

pub type Name = String;

#[derive(Bundle)]
pub struct ActorBundle {
    pub name: Name,
    pub actor: Actor,
    pub activity: Activity,
    pub position: Position,
    pub viewshed: Viewshed,
}

impl Default for ActorBundle {
    fn default() -> Self {
        Self {
            name: "Missing name!".to_string(),
            actor: Default::default(),
            activity: Default::default(),
            position: Position::zero(),
            viewshed: Default::default(),
        }
    }
}
