use bevy::{
    math::IVec2,
    prelude::{Bundle, Color, Component},
};

use crate::{core::types::GridPos, world::Renderable};

#[derive(Debug, Component, Default)]
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
            position: GridPos(IVec2::default()),
            renderable: Renderable {
                glyph: '/',
                fg: Color::YELLOW,
                bg: Color::DARK_GRAY,
            },
        }
    }
}
