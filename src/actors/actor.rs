use bevy::{
    math::IVec2,
    prelude::{Bundle, Color, Component},
};

use crate::{
    core::types::{Cardinal, GridPos, Int, Percentage},
    world::{Renderable, Viewshed},
};

const BASE_HEALTH: Int = 100;
const BASE_SPEED: Int = 100;

#[derive(Debug, Component)]
pub struct CharacterSheet {
    pub health: Int, // maybe derive health from attributes
    pub speed: Int,
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

#[derive(Debug, Default, Component)]
pub struct CharacterProperties {
    pub health: Percentage,
    pub energy: Int, // Available time units
}

#[derive(Debug, Component)]
pub struct Actor {
    pub sheet: CharacterSheet,
    pub properties: CharacterProperties,
    pub facing: Cardinal,
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            sheet: Default::default(),
            properties: Default::default(),
            facing: Cardinal::North,
        }
    }
}

#[derive(Bundle)]
pub struct ActorBundle {
    pub actor: Actor,
    pub position: GridPos,
    pub viewshed: Viewshed,
    pub renderable: Renderable,
}

impl Default for ActorBundle {
    fn default() -> Self {
        Self {
            actor: Default::default(),
            position: GridPos(IVec2::default()),
            viewshed: Default::default(),
            renderable: Renderable {
                glyph: 'X',
                fg: Color::YELLOW,
                bg: Color::DARK_GRAY,
            },
        }
    }
}
