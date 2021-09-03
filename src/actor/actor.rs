use crate::types::{Facing, Percentage};

use super::action::Action;

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
    pub action: Action,
    pub facing: Facing,
}

#[derive(Debug)]
pub struct Name {
    pub name: String,
}
