pub use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_graph::{base, AssetRenderResourcesNode};
use bevy::render::{
    mesh::VertexAttributeValues,
    pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{RenderGraph, RenderResourcesNode},
    renderer::RenderResources,
    shader::{ShaderStage, ShaderStages},
};

use crate::util::helpers::colors::greyscale;
use crate::{
    core::types::{GridPos, Int, Real, RealPos},
    world::{self, AreaGrid},
};

const TILE_SIZE: Int = 16;

pub struct Grid;

pub fn draw(
    map_query: Query<&AreaGrid, Changed<AreaGrid>>,
    mut query: Query<&Children, With<Grid>>,
    mut tile_query: Query<(&mut Handle<Mesh>, &mut TextureAtlasSprite, &Handle<MyTile>)>,
    mut cp437_assets: ResMut<Assets<MyTile>>,
) {
    if map_query.is_empty() {
        return;
    }
    let map = map_query.single();

    let children = query.single_mut();

    assert!(!map.tiles.is_empty());
    for (idx, tile) in map.tiles.iter().enumerate() {
        let tile_entity = children[idx];
        let (mut mesh_handle, mut sprite, handle) = tile_query.get_mut(tile_entity).unwrap();

        let (x, y) = map.idx_xy(idx);
        let pos = GridPos::new(x, y);

        let mut index = match tile {
            world::TileType::Wall => 35_u32,
            world::TileType::Floor => 46_u32,
        };
        let mut fg = Color::ORANGE;
        let mut bg = Color::SEA_GREEN;
        if map.visible[idx] {
            if let Some(renderable) = map.renderables.get(&pos) {
                index = renderable.glyph;
                fg = renderable.fg;
                bg += renderable.bg;
            }
        }
        sprite.index = index;

        if !map.revealed[idx] {
            fg = Color::BLACK;
            bg = Color::BLACK;
        } else if !map.visible[idx] {
            fg = greyscale(&fg);
            bg = greyscale(&bg);
        }

        if let Some(mut cp_tile) = cp437_assets.get_mut(handle) {
            cp_tile.fg = fg;
            cp_tile.bg = bg;
        }

        // if !map.revealed[idx] {
        //     sprite.color = Color::BLACK;
        // } else if !map.visible[idx] {
        //     sprite.color = Color::GRAY;
        // } else {
        //     sprite.color = Color::WHITE;
        // }
    }
    println!("Done drawing");
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "93fb26fc-6c05-489b-9029-601edf703b6b"]
pub struct CP437TilesetTexture {
    pub texture: Handle<Texture>,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "620f651b-adbe-464b-b740-ba0e547282ba"]
pub struct MyTile {
    pub fg: Color,
    pub bg: Color,
}

struct LoadingTexture(Option<Handle<Texture>>);

struct CP437Pipeline(Handle<PipelineDescriptor>);

pub fn load_char_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_query: Query<&AreaGrid>,
    window: Res<WindowDescriptor>,

    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,

    mut cp437_assets: ResMut<Assets<MyTile>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map = map_query.single();

    // TODO: load as a uniform sampler 2d
    // derive tile from cp437 index
    let texture_handle = asset_server.load("16x16-RogueYun-AgmEdit.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut children: Vec<Entity> = vec![];

    // Create a new shader pipeline.
    let pipeline = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            crate::rendering::sprite_sheet_shaders::VERTEX_SHADER,
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            crate::rendering::sprite_sheet_shaders::FRAGMENT_SHADER,
        ))),
    }));

    // commands.insert_resource(CP437Pipeline(pipeline));

    // Start loading the texture.
    // commands.insert_resource(LoadingTexture(Some(
    // let cp_tileset: Handle<Texture> = asset_server.load("16x16-RogueYun-AgmEdit.png");
    // )));

    // // Add an AssetRenderResourcesNode to our Render Graph. This will bind CP437TilesetTexture resources
    // // to our shader.
    render_graph.add_system_node("my_tile", AssetRenderResourcesNode::<MyTile>::new(true));
    // Add a Render Graph edge connecting our new "my_array_texture" node to the main pass node.
    // This ensures "cp437_tileset_texture" runs before the main pass.
    render_graph
        .add_node_edge("my_tile", base::node::MAIN_PASS)
        .unwrap();

    for index in 0..map.tiles.len() {
        let (x, y) = map.idx_xy(index);
        let pos = RealPos::new(
            -window.width / 2.0 + (x * TILE_SIZE + TILE_SIZE / 2) as Real,
            window.height / 2.0 - (y * TILE_SIZE + TILE_SIZE / 2) as Real,
        );
        let cp_tile_handle = cp437_assets.add(MyTile {
            fg: Color::GOLD,
            bg: Color::DARK_GRAY,
        });
        children.push(spawn_sprite_tile(
            &mut commands,
            pos,
            pipeline.clone(),
            texture_atlas_handle.clone(),
            cp_tile_handle,
        ));
    }

    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Grid)
        .insert_children(0, &children);
}

fn spawn_sprite_tile(
    commands: &mut Commands,
    pos: RealPos,
    pipeline: Handle<PipelineDescriptor>,
    texture_atlas_handle: Handle<TextureAtlas>,
    cp_437: Handle<MyTile>,
) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: pos.extend(0.0),
                ..Default::default()
            },
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline)]),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(cp_437)
        .id()
}
