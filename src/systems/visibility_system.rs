use crate::components::{Player, Position, Viewshed};
use crate::Map;
use bevy_ecs::prelude::*;
use rltk::{field_of_view, Point};

pub fn update_viewsheds(map: ResMut<Map>, mut query: Query<(&mut Viewshed, &Position)>) {
    println!("Updating viewsheds");
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

pub fn update_player_viewshed(mut map: ResMut<Map>, mut query: Query<&mut Viewshed, With<Player>>) {
    println!("Updating player viewshed");
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
