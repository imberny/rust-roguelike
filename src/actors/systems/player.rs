use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};

use crate::{
    actors::{Action, Activity, Player},
    settings::PlayerSettings,
    AppState,
};

pub fn handle_player_input(
    mut commands: Commands,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    settings: Res<PlayerSettings>,
    mut app_state: ResMut<State<AppState>>,
    player_query: Query<Entity, (With<Player>, Without<Activity>)>,
) {
    let mut success = false;
    keyboard_input_events.iter().for_each(|input| {
        player_query.iter().for_each(|player_ent| {
            if let Some(action) = try_into_action(input, &settings) {
                success = true;

                commands.entity(player_ent).insert(Activity {
                    time_to_complete: 30,
                    action,
                });
            }
        });
    });

    //TODO: replace with system checking if player is idle
    if success {
        println!("Running");
        app_state.set(AppState::Running).unwrap();
    }
}

fn try_into_action(keyboard_input: &KeyboardInput, settings: &PlayerSettings) -> Option<Action> {
    if keyboard_input.state == ElementState::Pressed
        && settings
            .input_map
            .contains_key(&keyboard_input.key_code.unwrap())
    {
        Some(settings.input_map[&keyboard_input.key_code.unwrap()])
    } else {
        None
    }
}
