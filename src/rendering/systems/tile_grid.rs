use bevy::prelude::*;

use crate::{
    core::types::GridPos,
    rendering::{grid::Grid, CP437Tile},
    util::helpers::{colors::greyscale, cp437},
    world::{AreaGrid, TileType},
};

pub fn draw(
    map_query: Query<&AreaGrid, Changed<AreaGrid>>,
    mut query: Query<&Children, With<Grid>>,
    mut tile_query: Query<(&mut TextureAtlasSprite, &mut CP437Tile)>,
) {
    if map_query.is_empty() {
        return;
    }
    let map = map_query.single();

    let children = query.single_mut();

    assert!(!map.tiles.is_empty());
    for (idx, tile) in map.tiles.iter().enumerate() {
        let tile_entity = children[idx];
        let (mut sprite, mut cp_tile) = tile_query.get_mut(tile_entity).unwrap();

        let (x, y) = map.idx_xy(idx);
        let pos = GridPos(IVec2::new(x, y));

        let mut index = match tile {
            TileType::Wall => 35_u32,
            TileType::Floor => 46_u32,
        };
        let mut fg = Color::ORANGE;
        let mut bg = Color::SEA_GREEN;
        if map.visible[idx] {
            if let Some(renderable) = map.renderables.get(&pos.0) {
                index = cp437(renderable.glyph);
                fg = renderable.fg;
                bg = renderable.bg;
            }
        }
        sprite.index = index;

        if !map.revealed[idx] {
            fg = Color::BLACK;
            bg = Color::BLACK;
        } else if !map.visible[idx] {
            fg = greyscale(&fg);
            bg = greyscale(&bg);
        }

        cp_tile.fg = fg;
        cp_tile.bg = bg;
    }
}
