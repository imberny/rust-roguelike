use bevy_ecs::prelude::*;
use rltk::{field_of_view, Point};

use crate::{actor::Viewshed, map::Map, types::Position};

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
