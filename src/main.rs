use crate::actors::systems::is_player_busy;
use crate::core::types::{Int, Real};
use crate::core::{advance_time, IncrementalClock, TurnGameStage};
use crate::game_world::AreaGrid;
use crate::generator::{generate_map_system, MapGenerator};

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
        .init_resource::<AreaGrid>()
        // .insert_resource(context)
        .add_startup_system(generate_map_system.label(SystemLabels::Generation))
        .add_startup_system(load_char_tiles.after(SystemLabels::Generation))
        .add_system(draw.system())
        .insert_resource(IncrementalClock::default())
        .add_state(AppState::Paused)
        // .add_system(advance_time.system())
        // .add_plugin(actors::ActorPlugin)
        // .add_plugin(ai::AIPlugin)
        .add_plugin(game_world::GameWorldPlugin)
        // .add_system(render_sys.system())
        .run();
}

fn draw(
    mut commands: Commands,
    map: Res<AreaGrid>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<&Children, With<Grid>>,
    mut tile_query: Query<&mut TextureAtlasSprite>,
    // map_query: Query<&AreaGrid, Changed<AreaGrid>>
) {
    // let map = map_query.single().unwrap();

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

        if map.revealed[idx] {
            sprite.color = Color::WHITE;
        } else {
            sprite.color = Color::BLACK;
        }
    }
}

struct Grid;

fn load_char_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<AreaGrid>,
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
                            (-WIDTH / 2 + x * 16 + 8) as f32,
                            (-HEIGHT / 2 + y * 16 + 8) as f32,
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
