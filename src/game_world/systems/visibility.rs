use bevy_ecs::prelude::*;

use crate::{
    actors::Actor,
    core::types::GridPos,
    game_world::{
        field_of_view::{new_quadratic, FieldOfView},
        AreaGrid, Viewshed,
    },
};

use super::shadow_casting::SymmetricShadowcaster;

pub fn update_viewsheds(
    map: ResMut<AreaGrid>,
    mut query: Query<(&mut Viewshed, &GridPos, &Actor)>,
) {
    for (mut viewshed, pos, actor) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;

            let fov = new_quadratic(15, actor.facing, 0.5, -1.5);
            viewshed.visible_tiles = SymmetricShadowcaster::new(&map)
                .visible_positions(pos.clone(), |pos| fov.sees(pos));
        }
    }
}