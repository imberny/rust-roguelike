use bevy::prelude::{Color, Component};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}
