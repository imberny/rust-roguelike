use bevy_ecs::prelude::*;
use rltk::Point;

use crate::{
    core::types::Position,
    game_world::{field_of_view::FieldOfView, AreaGrid, Viewshed},
};

use super::shadow_casting::SymmetricShadowcaster;

pub fn update_viewsheds(map: ResMut<AreaGrid>, mut query: Query<(&mut Viewshed, &Position)>) {
    for (mut viewshed, pos) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles = SymmetricShadowcaster::new(&map, FieldOfView::infinite())
                .visible_positions(Point::new(pos.x, pos.y));
        }
    }
}
