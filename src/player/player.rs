use crate::{
    actor::Actor, map::TileType, types::Facing, Game, Map, Player,
    PlayerInput, Position, TurnBasedState, Viewshed,
};
use bevy_ecs::prelude::*;
use std::cmp::{max, min};

use super::input::input_to_action;

// fn try_move_player(
//     movement_delta: Facing,
//     is_strafing: bool,
//     map: Res<Map>,
//     mut player_query: Query<(&mut Position, &mut Viewshed, &mut Actor), With<Player>>,
// ) -> bool {

//     let mut success = false;

//     for (mut pos, mut viewshed, mut actor) in player_query.iter_mut() {
//         let mut actual_move = movement_delta.clone();
//         if movement_delta != actor.facing && !is_strafing {
//             actual_move = IDLE_MOVE;
//             actor.facing = movement_delta;
//         }

//         let destination_idx = map.xy_idx(pos.x + actual_move.x, pos.y + actual_move.y);
//         let destination_tile = map.tiles[destination_idx];
//         if destination_tile != TileType::Wall {
//             pos.x = min(79, max(0, pos.x + actual_move.x));
//             pos.y = min(49, max(0, pos.y + actual_move.y));

//             viewshed.dirty = true;
//             success = true;
//         }
//     }
//     success
// }

// pub fn handle_input(
//     mut game: ResMut<Game>,
//     input: Res<PlayerInput>,
//     map: Res<Map>,
//     player_query: Query<(&mut Position, &mut Viewshed, &mut Actor), With<Player>>,
// ) {
//     let is_strafing = input.is_strafing;
//     let direction = input_to_action(input);

//     println!("Processing input");
//     if let Some(movement_delta) = direction {
//         if try_move_player(movement_delta, is_strafing, map, player_query) {
//             game.turn_based_state = TurnBasedState::OpponentsTurn;
//         }
//     }
// }

pub fn handle_player_input(
    input: Res<PlayerInput>,
    mut player_query: Query<&mut Actor, With<Player>>,
) {
    for mut player in player_query.iter_mut() {
        player.action = input.action;
    }
}
