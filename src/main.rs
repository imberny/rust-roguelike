use crate::core::systems::advance_time;

use bevy::prelude::*;

use crate::{
    actors::{Activity, ActorSystems, Player},
    core::{
        types::{Int, Real},
        IncrementalClock, TimeIncrementEvent,
    },
    settings::PlayerSettings,
    world::{AreaGrid, TileType},
};

mod actors;
mod ai;
mod core;
mod rendering;
mod settings;
mod util;
mod world;

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
    App::new()
        .insert_resource(WindowDescriptor {
            title: "The Possession of Barbe Halle".to_string(),
            width: WIDTH as Real,
            height: HEIGHT as Real,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .init_resource::<IncrementalClock>()
        .init_resource::<PlayerSettings>()
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, set_up_map)
        .add_event::<TimeIncrementEvent>()
        .add_state(AppState::Running)
        .add_system_set(
            SystemSet::on_update(AppState::Running)
                .with_system(advance_time)
                .before(ActorSystems::Action),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Running)
                .with_system(pause_if_player_idle.after(ActorSystems::Action)),
        )
        .add_plugin(actors::ActorPlugin)
        .add_plugin(ai::AIPlugin)
        .add_plugin(world::GameWorldPlugin)
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
    player_query: Query<(), (With<Player>, Without<Activity>)>,
) {
    if player_query.is_empty() {
        return;
    }
    println!("Player idle");
    if *app_state.current() == AppState::Running {
        app_state.set(AppState::Paused).unwrap()
    }
}
