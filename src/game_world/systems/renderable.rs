use bevy::prelude::*;

use crate::{core::types::GridPos, game_world::AreaGrid, rendering::Renderable};

pub fn update_renderables(
    mut map_query: Query<&mut AreaGrid>,
    query: Query<(&GridPos, &Renderable)>,
) {
    let mut map = map_query.single_mut();
    map.renderables.drain();
    query.iter().for_each(|(pos, renderable)| {
        map.renderables.insert(pos.clone(), renderable.clone());
    });
}
