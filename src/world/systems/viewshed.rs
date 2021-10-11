use bevy::prelude::*;

use crate::{
    actors::{Actor, Player},
    core::types::GridPos,
    util::algorithms::{field_of_view::FOV, symmetric_shadowcasting},
    world::{Viewshed, WorldMap},
};

pub fn update_viewsheds(
    world_map: ResMut<WorldMap>,
    mut query: Query<(&mut Viewshed, &GridPos, &Actor)>,
) {
    for (mut viewshed, pos, actor) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;

            let area = &world_map.get_area_from_pos(&pos.0).unwrap().1;
            let fov = FOV::Quadratic(15, 0.5, -1.5);
            viewshed.visible_tiles =
                symmetric_shadowcasting(&pos.0, &|pos| fov.sees(pos, actor.facing), &|pos| {
                    area.is_blocking(pos)
                });
        }
    }
}

pub fn apply_player_viewsheds(
    mut world_map: ResMut<WorldMap>,
    query: Query<(&GridPos, &Viewshed), With<Player>>,
) {
    let (pos, viewshed) = query.single();
    let area = &mut world_map.get_area_from_pos_mut(&pos.0).unwrap().1;

    for t in area.visible.iter_mut() {
        *t = false
    }
    for visible_position in &viewshed.visible_tiles {
        let mut tile = area.tile_at_mut(visible_position).unwrap();
        tile.set_visible(true);
        tile.set_revealed(true);
    }
}
