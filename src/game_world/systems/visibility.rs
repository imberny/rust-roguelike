use bevy_ecs::prelude::*;

use crate::{
    actor::Actor,
    core::types::Position,
    game_world::{field_of_view::new_quadratic, AreaGrid, Viewshed},
};

use super::shadow_casting::SymmetricShadowcaster;

pub fn update_viewsheds(
    map: ResMut<AreaGrid>,
    mut query: Query<(&mut Viewshed, &Position, &Actor)>,
) {
    for (mut viewshed, pos, actor) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles = SymmetricShadowcaster::new(&map)
                .visible_positions(pos.clone(), new_quadratic(15, actor.facing, 0.5, -1.5));
        }
    }
}
