use bevy::prelude::*;

use crate::{
    actors::Actor,
    core::{
        types::{Facing, GridPos, Int},
        MainPointOfView,
    },
    rendering::{
        constants::WORLD_VIEWPORT_DIMENSIONS, draw_event::CP437TileInfo, grid::Grid, CP437Tile,
        DrawEvent,
    },
    util::helpers::{colors::greyscale, cp437},
    world::{AreaGrid, TileType, WorldMap},
};

pub fn pre_draw(
    world_map: Res<WorldMap>,
    grid_query: Query<&Children, With<Grid>>,
    pov_query: Query<(&GridPos, &Actor), With<MainPointOfView>>,
    mut draw_event_writer: EventWriter<DrawEvent>,
) {
    let (camera_pos, actor) = pov_query.single();
    let camera_cardinal = actor.facing;

    let offset_area = world_map.get_area_from_pos(&camera_pos.0).unwrap();
    let offset = &offset_area.0;
    let area = &offset_area.1;

    let mut draw_map: Vec<CP437TileInfo> = vec![];
    let (columns, rows) = WORLD_VIEWPORT_DIMENSIONS;
    draw_map.reserve(columns * rows);

    let viewport_tiles = grid_query.single();

    let (min_x, max_x) = (offset.x, offset.x + area.width - 1);
    let (min_y, max_y) = (offset.y, offset.y + area.height - 1);
    let top_left = IVec2::new(
        (camera_pos.0.x - (columns / 2) as Int).clamp(min_x, max_x),
        (camera_pos.0.y - (rows / 2) as Int).clamp(min_y, max_y),
    );

    (0..viewport_tiles.len()).for_each(|index| {
        let y = (index % columns) as Int;
        let x = (index / columns) as Int;
        let pos = IVec2::new(
            (top_left.x + x).clamp(min_x, max_x),
            (top_left.y + y).clamp(min_y, max_y),
        );

        let tile = area.tile_at(&pos).unwrap();
        let mut sprite_index = match tile.which() {
            TileType::Wall => 35_u32,
            TileType::Floor => 46_u32,
        };

        let mut fg = Color::ORANGE;
        let mut bg = Color::SEA_GREEN;

        if tile.is_visible() {
            if let Some(renderable) = area.renderables.get(&pos) {
                sprite_index = cp437(renderable.glyph);
                fg = renderable.fg;
                bg = renderable.bg;
            }
        } else if !tile.is_revealed() {
            fg = Color::BLACK;
            bg = Color::BLACK;
        } else {
            fg = greyscale(&fg);
            bg = greyscale(&bg);
        }
        draw_map.push(CP437TileInfo {
            sprite_index,
            fg,
            bg,
        })
    });

    draw_event_writer.send(DrawEvent { tiles: draw_map });
}

pub fn draw(
    mut query: Query<&Children, With<Grid>>,
    mut tile_query: Query<(&mut TextureAtlasSprite, &mut CP437Tile)>,
    mut draw_event_reader: EventReader<DrawEvent>,
) {
    let children = query.single_mut();

    for draw_event in draw_event_reader.iter() {
        draw_event
            .tiles
            .iter()
            .enumerate()
            .for_each(|(index, cp437_tile_info)| {
                let tile_entity = children[index];
                let (mut sprite, mut cp_tile) = tile_query.get_mut(tile_entity).unwrap();
                sprite.index = cp437_tile_info.sprite_index;
                cp_tile.fg = cp437_tile_info.fg;
                cp_tile.bg = cp437_tile_info.bg;
            });
    }
}
