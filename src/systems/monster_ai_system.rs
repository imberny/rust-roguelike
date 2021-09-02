use crate::{Monster, Name, Viewshed, components::{Player, Position}};
use bevy_ecs::prelude::*;
use rltk::{console};

pub fn monster_ai(
    monster_query: Query<(&Viewshed, &Name), With<Monster>>,
    player_query: Query<&Position, With<Player>>,
) {
    println!("processing monster ai");
    for (viewshed, name) in monster_query.iter() {
        for player_pos in player_query.iter() {
            if viewshed.visible_tiles.contains(player_pos) {
                console::log(format!("{} shouts insults at the player.", name.name));
            }
        }
        
    }
}
