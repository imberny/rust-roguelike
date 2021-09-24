use crate::{
    core::{
        constants::*,
        types::{Facing, GridPos, Percentage},
    },
    game_world::Viewshed,
};
use bevy_ecs::bundle::Bundle;

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

#[derive(Debug)]
pub struct Actor {
    pub sheet: CharacterSheet,
    pub properties: CharacterProperties,
    pub facing: Facing,
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            sheet: Default::default(),
            properties: Default::default(),
            facing: SOUTH,
        }
    }
}

pub type Name = String;

#[derive(Bundle)]
pub struct ActorBundle {
    pub name: Name,
    pub actor: Actor,
    pub position: GridPos,
    pub viewshed: Viewshed,
}

impl Default for ActorBundle {
    fn default() -> Self {
        Self {
            name: "Missing name!".to_string(),
            actor: Default::default(),
            position: GridPos::default(),
            viewshed: Default::default(),
        }
    }
}
