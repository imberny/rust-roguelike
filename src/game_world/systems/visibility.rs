use bevy_ecs::prelude::*;

use crate::{
    actors::Actor,
    core::types::GridPos,
    game_world::{
        field_of_view::{new_quadratic, FieldOfView},
        AreaGrid, Viewshed,
    },
};

use super::shadow_casting::symmetric_shadowcasting;

pub fn update_viewsheds(
    map: ResMut<AreaGrid>,
    mut query: Query<(&mut Viewshed, &GridPos, &Actor)>,
) {
    let map_clone = map.clone();
    for (mut viewshed, pos, actor) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;

            let fov = new_quadratic(15, actor.facing, 0.5, -1.5);
            viewshed.visible_tiles =
                symmetric_shadowcasting(pos.clone(), &|pos| fov.sees(pos), &|pos| {
                    map_clone.is_blocking(pos)
                });
        }
    }
}
