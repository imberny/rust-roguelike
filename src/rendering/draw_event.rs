use bevy::prelude::Color;

pub struct CP437TileInfo {
    pub sprite_index: u32,
    pub fg: Color,
    pub bg: Color,
}

pub struct DrawEvent {
    pub tiles: Vec<CP437TileInfo>,
}
