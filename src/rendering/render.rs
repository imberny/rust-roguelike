use bevy_ecs::prelude::*;
use rltk::{Rltk, RGB};

use crate::{
    actors::Actor,
    core::types::{Facing, GridPos, IntoGridPos, RealPos},
    game_world::{self, AreaGrid},
};

use super::Renderable;

pub fn render(world: &mut World, ctx: &mut Rltk) {
    ctx.cls();
    draw_map(world, ctx);
    draw_entities(world, ctx);
}

fn draw_map(world: &World, ctx: &mut Rltk) {
    let map = world.get_resource::<AreaGrid>().unwrap();
    for (idx, tile) in map.tiles.iter().enumerate() {
        if !map.revealed[idx] {
            continue;
        }

        let (x, y) = map.idx_xy(idx);
        let mut fg;
        let glyph;
        match tile {
            game_world::TileType::Floor => {
                fg = RGB::from_f32(0.0, 0.5, 0.5);
                glyph = rltk::to_cp437('.');
            }
            game_world::TileType::Wall => {
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
    let mut renderables = world.query::<(&GridPos, &Renderable, &Actor)>();
    let map = world.get_resource::<AreaGrid>().unwrap();
    renderables.for_each(world, |(pos, render, actor)| {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible[idx] {
            let facing: Facing = actor.facing.into();
            let weapon_position =
                (RealPos::from(*pos) - facing.reversed() * RealPos::new(0.0, -1.0)).as_grid_pos();
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            ctx.set(
                weapon_position.x,
                weapon_position.y,
                render.fg,
                render.bg,
                rltk::to_cp437('\\'),
            );
        }
    });
}
