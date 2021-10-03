use crate::actors::systems::is_player_busy;
use crate::actors::ActorSystems;
use crate::core::types::{Int, Real};
use crate::core::{advance_time, IncrementalClock, TimeIncrementEvent, TurnGameStage};
use crate::game_world::AreaGrid;
use crate::generator::{generate_map_system, MapGenerator};

use actors::{Activity, Player};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
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
        // // Adds frame time diagnostics
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // // Adds a system that prints diagnostics to the console
        // .add_plugin(LogDiagnosticsPlugin::default())
        .init_resource::<AreaGrid>()
        .add_event::<TimeIncrementEvent>()
        // .insert_resource(context)
        .add_state(AppState::Running)
        .add_startup_system(generate_map_system.label(SystemLabels::Generation))
        .insert_resource(IncrementalClock::default())
        .add_system_set(SystemSet::on_exit(AppState::Paused).with_system(advance_time))
        .add_system(pause_if_player_idle.after(ActorSystems::Action))
        .add_plugin(actors::ActorPlugin)
        .add_plugin(ai::AIPlugin)
        .add_plugin(game_world::GameWorldPlugin)
        .add_plugin(rendering::TileRendererPlugin)
        // .add_system(render_sys.system())
        .run();
}

fn pause_if_player_idle(
    mut app_state: ResMut<State<AppState>>,
    player_query: Query<&Player, Without<Activity>>,
) {
    if *app_state.current() == AppState::Running {
        app_state.set(AppState::Rendering).unwrap()
    }
}
