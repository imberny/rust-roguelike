pub use bevy::prelude::*;

use crate::{
    core::types::{GridPos, Int, Real},
    game_world::{self, AreaGrid},
    rendering::cp437,
    util::helpers::colors::greyscale,
    AppState,
};

const TILE_SIZE: Int = 16;

pub struct Grid;

pub fn draw(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    // map: Res<AreaGrid>,
    map_query: Query<&AreaGrid>,
    mut query: Query<&Children, With<Grid>>,
    mut tile_query: Query<&mut TextureAtlasSprite>,
    // map_query: Query<&AreaGrid, Changed<AreaGrid>>
) {
    let map = map_query.single();

    println!("Drawing");

    let children = query.single_mut();

    assert!(!map.tiles.is_empty());
    for (idx, tile) in map.tiles.iter().enumerate() {
        let tile_entity = children[idx];
        let mut sprite = tile_query.get_mut(tile_entity).unwrap();

        let (x, y) = map.idx_xy(idx);
        let pos = GridPos::new(x, y);

        if let Some(renderable) = map.renderables.get(&pos) {
            sprite.index = renderable.glyph;
        } else {
            sprite.index = match tile {
                game_world::TileType::Wall => 35,
                game_world::TileType::Floor => 46,
            };
        }

        // let mut fg = renderable.fg;
        // if !map.visible[idx] {
        //     fg = greyscale(&fg);
        // }

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

pub fn load_char_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // map: Res<AreaGrid>,
    map_query: Query<&AreaGrid>,
    window: Res<WindowDescriptor>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map = map_query.single();

    let texture_handle = asset_server.load("16x16-RogueYun-AgmEdit.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut children: Vec<Entity> = vec![];

    for index in 0..map.tiles.len() {
        let (x, y) = map.idx_xy(index);
        children.push(
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            -window.width / 2.0 + (x * TILE_SIZE + TILE_SIZE / 2) as Real,
                            window.height / 2.0 - (y * TILE_SIZE + TILE_SIZE / 2) as Real,
                            0.,
                        ),
                        ..Default::default()
                    },
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
