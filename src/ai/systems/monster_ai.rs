use bevy_ecs::prelude::*;

use crate::{
    actors::{Action, Activity, Player},
    ai::Monster,
    core::types::GridPos,
    game_world::Viewshed,
};

pub fn monster_ai(
    mut commands: Commands,
    mut monster_query: Query<(Entity, &Viewshed), (With<Monster>, Without<Activity>)>,
    player_query: Query<&GridPos, With<Player>>,
) {
    for (monster, viewshed) in monster_query.iter_mut() {
        for player_pos in player_query.iter() {
            if viewshed.visible_tiles.contains(player_pos) {
                commands.entity(monster).insert(Activity {
                    time_to_complete: 60,
                    action: Action::InitiateAttack,
                });
            }
        }
    }
}
