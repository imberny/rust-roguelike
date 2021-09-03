use bevy_ecs::prelude::*;
use rltk::{field_of_view, Point};

use crate::{actor::Viewshed, map::Map, player::Player, types::Position};

pub fn update_viewsheds(map: ResMut<Map>, mut query: Query<(&mut Viewshed, &Position)>) {
    for (mut viewshed, pos) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| map.is_in_bounds(p.x, p.y));
        }
    }
}

pub fn apply_player_viewsheds(mut map: ResMut<Map>, mut query: Query<&mut Viewshed, With<Player>>) {
    for t in map.visible.iter_mut() {
        *t = false
    }
    for viewshed in query.iter_mut() {
        for vis in &viewshed.visible_tiles {
            let idx = map.xy_idx(vis.x, vis.y);
            map.revealed[idx] = true;
            map.visible[idx] = true;
        }
    }
}
