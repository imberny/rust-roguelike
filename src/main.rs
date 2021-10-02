use crate::actors::systems::is_player_busy;
use crate::actors::ActorSystems;
use crate::core::types::{Int, Real};
use crate::core::{advance_time, IncrementalClock, TimeIncrementEvent, TurnGameStage};
use crate::game_world::AreaGrid;
use crate::generator::{generate_map_system, MapGenerator};

use actors::{Activity, Player};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use rendering::render_sys;
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
    use rltk::RltkBuilder;
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
        .add_startup_system(load_char_tiles.after(SystemLabels::Generation))
        .add_system_set(SystemSet::on_enter(AppState::Rendering).with_system(draw.system()))
        .insert_resource(IncrementalClock::default())
        .add_system_set(SystemSet::on_enter(AppState::Running).with_system(advance_time))
        .add_system(pause_if_player_idle.after(ActorSystems::Action))
        .add_plugin(actors::ActorPlugin)
        // .add_plugin(ai::AIPlugin)
        .add_plugin(game_world::GameWorldPlugin)
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

fn draw(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    map: Res<AreaGrid>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<&Children, With<Grid>>,
    mut tile_query: Query<&mut TextureAtlasSprite>,
    // map_query: Query<&AreaGrid, Changed<AreaGrid>>
) {
    // let map = map_query.single().unwrap();

    println!("Drawing");

    let children = query.single_mut().unwrap();

    assert!(map.tiles.len() > 0);
    for (idx, tile) in map.tiles.iter().enumerate() {
        let renderable = map.renderables[idx];
        let mut fg = renderable.fg;
        if !map.visible[idx] {
            fg = fg.to_greyscale();
        }

        // let (x, y) = map.idx_xy(idx);

        let tile_entity = children[idx];
        let mut sprite = tile_query.get_mut(tile_entity).unwrap();

        if !map.revealed[idx] {
            sprite.color = Color::BLACK;
        } else if !map.visible[idx] {
            sprite.color = Color::GRAY;
        } else {
            sprite.color = Color::WHITE;
        }
    }
    app_state.set(AppState::Paused).unwrap();
    println!("Done drawing");
}

struct Grid;

fn load_char_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<AreaGrid>,
    window: Res<WindowDescriptor>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("16x16-RogueYun-AgmEdit.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut children: Vec<Entity> = vec![];

    for (index, tile) in map.tiles.iter().enumerate() {
        let (x, y) = map.idx_xy(index);
        let sprite_index = match tile {
            game_world::TileType::Wall => 35,
            game_world::TileType::Floor => 46,
        };
        children.push(
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            -window.width / 2.0 + ((x * 16 + 8) as f32),
                            window.height / 2.0 - ((y * 16 + 8) as f32),
                            0.,
                        ),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite::new(sprite_index),
                    ..Default::default()
                })
                .id(),
        );
    }

    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Grid)
        .insert_children(0, &children);
}
