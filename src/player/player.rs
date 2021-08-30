use crate::map::TileType;
use crate::{Game, Map, Player, PlayerInput, Position, TurnBasedState, Viewshed};
use bevy_ecs::prelude::*;
use std::cmp::{max, min};

use super::input::{direction_from, Direction};

fn try_move_player(
    delta: Direction,
    map: Res<Map>,
    mut player_query: Query<(&mut Position, &mut Viewshed), With<Player>>,
) -> bool {
    // let mut positions = ecs.write_storage::<Position>();
    // let mut players = ecs.write_storage::<Player>();
    // let mut viewsheds = ecs.write_storage::<Viewshed>();
    // let mut player_pos = ecs.fetch_mut::<PlayerPosition>();
    let mut success = false;
    for (mut pos, mut viewshed) in player_query.iter_mut() {
        let destination_idx = map.xy_idx(pos.x + delta.x, pos.y + delta.y);
        let destination_tile = map.tiles[destination_idx];
        if destination_tile != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta.x));
            pos.y = min(49, max(0, pos.y + delta.y));
            // player_pos.x = pos.x;
            // player_pos.y = pos.y;

            viewshed.dirty = true;
            success = true;
        }
    }
    success
}

pub fn handle_input(
    mut game: ResMut<Game>,
    input: Res<PlayerInput>,
    map: Res<Map>,
    player_query: Query<(&mut Position, &mut Viewshed), With<Player>>,
) {
    let direction = direction_from(input);

    if let Some(movement_delta) = direction {
        if try_move_player(movement_delta, map, player_query) {
            game.turn_based_state = TurnBasedState::OpponentsTurn;
        }
    }
}
