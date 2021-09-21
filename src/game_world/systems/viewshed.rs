use crate::{
    actor::player::Player,
    game_world::{AreaGrid, Viewshed},
};
use bevy_ecs::prelude::{Query, ResMut, With};

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
