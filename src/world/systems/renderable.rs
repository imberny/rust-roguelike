use bevy::prelude::*;

use crate::{
    core::types::GridPos,
    world::{Renderable, WorldMap},
};

pub fn update_renderables(mut world_map: ResMut<WorldMap>, query: Query<(&GridPos, &Renderable)>) {
    let map = &mut world_map.get_area_from_pos_mut(&IVec2::ZERO).unwrap().1;
    map.clear_renderables();
    query.iter().for_each(|(pos, renderable)| {
        let mut tile = map.tile_at_mut(&pos.0).unwrap();
        tile.set_renderable(renderable.clone());
    });
}
