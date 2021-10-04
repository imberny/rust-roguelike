use bevy::prelude::Color;

use crate::core::types::FontChar;

#[derive(Debug, Default, Clone, Copy)]
pub struct Renderable {
    pub glyph: FontChar,
    pub fg: Color,
    pub bg: Color,
}
