use bevy_ecs::prelude::*;

use crate::{
    actors::{Actor, Player},
    core::types::GridPos,
    game_world::{AreaGrid, Viewshed},
    util::algorithms::{field_of_view::FOV, symmetric_shadowcasting},
};

pub fn update_viewsheds(
    map: ResMut<AreaGrid>,
    mut query: Query<(&mut Viewshed, &GridPos, &Actor)>,
) {
    let map_clone = map.clone();
    for (mut viewshed, pos, actor) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;

            // let pattern = vec![
            //     // GridPos::new(-1, 1),
            //     // GridPos::new(0, 1),
            //     // GridPos::new(1, 1),
            //     // GridPos::new(-1, 0),
            //     // GridPos::new(-2, 0),
            //     // GridPos::new(0, 0),
            //     // GridPos::new(1, 0),
            //     // GridPos::new(2, 0),
            //     //
            //     GridPos::new(-1, -1),
            //     // GridPos::new(-2, -1),
            //     // GridPos::new(-3, -1),
            //     GridPos::new(0, -1),
            //     GridPos::new(1, -1),
            //     // GridPos::new(2, -1),
            //     // GridPos::new(3, -1),
            //     //
            //     GridPos::new(-1, -2),
            //     GridPos::new(-2, -2),
            //     GridPos::new(-3, -2),
            //     GridPos::new(0, -2),
            //     GridPos::new(1, -2),
            //     GridPos::new(2, -2),
            //     GridPos::new(3, -2),
            //     //
            //     // GridPos::new(-1, -3),
            //     // GridPos::new(-2, -3),
            //     // GridPos::new(-3, -3),
            //     // GridPos::new(0, -3),
            //     // GridPos::new(1, -3),
            //     // GridPos::new(2, -3),
            //     // GridPos::new(3, -3),
            // ];

            let fov = FOV::Quadratic(15, 0.5, -1.5);
            // let fov = field_of_view::pattern_fov(pattern, actor.facing);
            // let fov = field_of_view::cone_fov(3, std::f32::consts::PI / 4.0, actor.facing.into());
            viewshed.visible_tiles =
                symmetric_shadowcasting(pos.clone(), &|pos| fov.sees(pos, actor.facing), &|pos| {
                    map_clone.is_blocking(pos)
                });
        }
    }
}

pub fn apply_player_viewsheds(
    mut map: ResMut<AreaGrid>,
    mut query: Query<&mut Viewshed, With<Player>>,
) {
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
