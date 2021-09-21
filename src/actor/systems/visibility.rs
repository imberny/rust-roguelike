use bevy_ecs::prelude::*;
use rltk::Point;

use crate::{
    core::types::Position,
    game_world::{AreaGrid, Viewshed},
};

use super::shadow_casting::symmetric_shadowcasting;

pub fn update_viewsheds(map: ResMut<AreaGrid>, mut query: Query<(&mut Viewshed, &Position)>) {
    for (mut viewshed, pos) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles =
                symmetric_shadowcasting(&map, Point::new(pos.x, pos.y), viewshed.range as usize);
        }
    }
}
