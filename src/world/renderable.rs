use bevy::prelude::Color;

#[derive(Debug, Default, Clone, Copy)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}
