use bevy::prelude::{Bundle, Color};

use crate::{core::types::GridPos, world::Renderable};

#[derive(Debug, Default)]
pub struct Weapon;

#[derive(Debug, Bundle)]
pub struct WeaponBundle {
    pub weapon: Weapon,
    pub position: GridPos,
    pub renderable: Renderable,
}

impl Default for WeaponBundle {
    fn default() -> Self {
        Self {
            weapon: Default::default(),
            position: Default::default(),
            renderable: Renderable {
                glyph: '/',
                fg: Color::YELLOW,
                bg: Color::DARK_GRAY,
            },
        }
    }
}
