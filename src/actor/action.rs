use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use rltk::Point;

use crate::{
    components::{Position, Viewshed},
    map::{Map, TileType},
    types::Facing,
};

use super::Actor;

#[derive(Clone, Copy)]
pub enum Action {
    None,
    Move(Facing),
}

impl Default for Action {
    fn default() -> Self {
        Self::None
    }
}

fn try_move(
    pos: &mut Point,
    viewshed: &mut Viewshed,
    movement_delta: &Facing,
    map: &Res<Map>,
) {
    let destination_idx = map.xy_idx(pos.x + movement_delta.x, pos.y + movement_delta.y);
    let destination_tile = map.tiles[destination_idx];
    if destination_tile != TileType::Wall {
        pos.x = min(79, max(0, pos.x + movement_delta.x));
        pos.y = min(49, max(0, pos.y + movement_delta.y));

        viewshed.dirty = true;
    }
}

pub fn process_move_actions(
    map: Res<Map>,
    mut query: Query<(&mut Position, &mut Viewshed, &mut Actor)>,
) {
    for (mut pos, mut viewshed, actor) in query.iter_mut() {
        match &actor.action {
            Action::None => (),
            Action::Move(direction) => {
                try_move(&mut pos, &mut viewshed, &direction, &map);
            }
        }
    }
}
