use bevy_ecs::prelude::*;

use crate::{
    actors::Actor,
    core::types::GridPos,
    game_world::{AreaGrid, Viewshed},
    util::algorithms::{
        field_of_view::{self, FieldOfView},
        symmetric_shadowcasting,
    },
};

pub fn update_viewsheds(
    map: ResMut<AreaGrid>,
    mut query: Query<(&mut Viewshed, &GridPos, &Actor)>,
) {
    let map_clone = map.clone();
    for (mut viewshed, pos, actor) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;

            let fov = field_of_view::quadratic_fov(15, actor.facing.into(), 0.5, -1.5);
            viewshed.visible_tiles =
                symmetric_shadowcasting(pos.clone(), &|pos| fov.sees(pos), &|pos| {
                    map_clone.is_blocking(pos)
                });
        }
    }
}
