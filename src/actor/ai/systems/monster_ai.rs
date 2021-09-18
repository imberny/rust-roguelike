use bevy_ecs::prelude::*;

use crate::{
    actor::{
        action::{Message, MessageType},
        ai::Monster,
        player::Player,
        Action, Activity, Viewshed,
    },
    core::types::Position,
};

pub fn monster_ai(
    mut commands: Commands,
    mut monster_query: Query<(Entity, &Viewshed), (With<Monster>, Without<Activity>)>,
    player_query: Query<&Position, With<Player>>,
) {
    for (monster, viewshed) in monster_query.iter_mut() {
        for player_pos in player_query.iter() {
            if viewshed.visible_tiles.contains(player_pos) {
                let roll = rltk::RandomNumberGenerator::new().roll_dice(1, 3);
                let activity = if roll == 1 {
                    Activity {
                        time_to_complete: 32,
                        action: Action::Say(Message {
                            kind: MessageType::Compliment,
                        }),
                    }
                } else if roll == 2 {
                    Activity {
                        time_to_complete: 13,
                        action: Action::Say(Message {
                            kind: MessageType::Threaten,
                        }),
                    }
                } else {
                    Activity {
                        time_to_complete: 12,
                        action: Action::Say(Message {
                            kind: MessageType::Insult,
                        }),
                    }
                };
                commands.entity(monster).insert(activity);
            }
        }
    }
}
