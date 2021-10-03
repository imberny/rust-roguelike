use crate::actors::systems::is_player_busy;
use crate::actors::ActorSystems;
use crate::core::types::{Int, Real};
use crate::core::{advance_time, IncrementalClock, TimeIncrementEvent, TurnGameStage};
use crate::game_world::{AreaGrid, TileType};
use crate::generator::{generate_map_system, MapGenerator};

use actors::{Activity, Player};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::utils::HashMap;
// use game::run_game;

mod actors;
mod ai;
mod core;
mod game;
mod game_world;
mod generator;
mod rendering;
mod util;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Paused,
    Running,
    Rendering,
}

const WIDTH: Int = 1280;
const HEIGHT: Int = 800;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum SystemLabels {
    Generation,
}

fn main() {
    // let context = RltkBuilder::simple80x50()
    //     .with_dimensions(160, 100)
    //     .with_title("The Possession of Barbe Halle")
    //     .build()
    //     .unwrap();

    // let (map, _rooms) = MapGenerator {}.generate_new_map();

    App::new()
        .insert_resource(WindowDescriptor {
            title: "The Possession of Barbe Halle".to_string(),
            width: WIDTH as Real,
            height: HEIGHT as Real,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, set_up_map)
        .add_event::<TimeIncrementEvent>()
        .add_state(AppState::Running)
        .add_startup_system(generate_map_system.label(SystemLabels::Generation))
        .insert_resource(IncrementalClock::default())
        .add_system_set(SystemSet::on_exit(AppState::Paused).with_system(advance_time))
        .add_system(pause_if_player_idle.after(ActorSystems::Action))
        .add_plugin(actors::ActorPlugin)
        .add_plugin(ai::AIPlugin)
        .add_plugin(game_world::GameWorldPlugin)
        .add_plugin(rendering::TileRendererPlugin)
        .run();
}

fn set_up_map(mut commands: Commands) {
    commands.spawn().insert(AreaGrid {
        revealed: vec![false; 80 * 50],
        visible: vec![false; 80 * 50],
        tiles: vec![TileType::Wall; 80 * 50],
        width: 80,
        height: 50,
        ..Default::default()
    });
}

fn pause_if_player_idle(
    mut app_state: ResMut<State<AppState>>,
    player_query: Query<&Player, Without<Activity>>,
) {
    if *app_state.current() == AppState::Running {
        app_state.set(AppState::Rendering).unwrap()
    }
}
