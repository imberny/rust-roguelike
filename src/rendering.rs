use bevy_ecs::prelude::*;
use rltk::{Rltk, RGB};

use crate::{
    actor::Actor,
    components::{Position, Renderable},
    map::{self, Map},
};

pub fn render(world: &mut World, ctx: &mut Rltk) {
    ctx.cls();
    draw_map(world, ctx);
    draw_entities(world, ctx);
}

fn draw_map(world: &World, ctx: &mut Rltk) {
    let map = world.get_resource::<Map>().unwrap();
    for (idx, tile) in map.tiles.iter().enumerate() {
        if !map.revealed[idx] {
            continue;
        }

        let (x, y) = map.idx_xy(idx);
        let mut fg;
        let glyph;
        match tile {
            map::TileType::Floor => {
                fg = RGB::from_f32(0.0, 0.5, 0.5);
                glyph = rltk::to_cp437('.');
            }
            map::TileType::Wall => {
                fg = RGB::from_f32(0., 1.0, 0.);
                glyph = rltk::to_cp437('#');
            }
        }
        if !map.visible[idx] {
            fg = fg.to_greyscale();
        }
        ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
    }
}

fn draw_entities(world: &mut World, ctx: &mut Rltk) {
    let mut renderables = world.query::<(&Position, &Renderable, &Actor)>();
    let map = world.get_resource::<Map>().unwrap();
    for (pos, render, actor) in renderables.iter(&world) {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible[idx] {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            ctx.set(
                pos.x + actor.facing.x,
                pos.y + actor.facing.y,
                render.fg,
                render.bg,
                rltk::to_cp437('\\'),
            );
        }
    }
}
