pub use bevy::prelude::*;
use bevy::render::{
    mesh::VertexAttributeValues,
    pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{RenderGraph, RenderResourcesNode},
    shader::{ShaderStage, ShaderStages},
};

use crate::{
    core::types::{GridPos, Int, Real, RealPos},
    world::{self, AreaGrid},
};

use super::{FRAGMENT_SHADER, VERTEX_SHADER};

const TILE_SIZE: Int = 16;

pub struct Grid;

pub fn draw(
    mut meshes: ResMut<Assets<Mesh>>,
    map_query: Query<&AreaGrid, Changed<AreaGrid>>,
    mut query: Query<&Children, With<Grid>>,
    mut tile_query: Query<(&mut Handle<Mesh>, &mut TextureAtlasSprite)>,
) {
    if map_query.is_empty() {
        return;
    }
    let map = map_query.single();

    let children = query.single_mut();

    assert!(!map.tiles.is_empty());
    for (idx, tile) in map.tiles.iter().enumerate() {
        let tile_entity = children[idx];
        let (mut mesh_handle, mut sprite) = tile_query.get_mut(tile_entity).unwrap();

        let (x, y) = map.idx_xy(idx);
        let pos = GridPos::new(x, y);

        if let Some(renderable) = map.renderables.get(&pos) {
            sprite.index = renderable.glyph;
        } else {
            sprite.index = match tile {
                world::TileType::Wall => 35,
                world::TileType::Floor => 46,
            };
        }

        // let mut fg = renderable.fg;
        // if !map.visible[idx] {
        //     fg = greyscale(&fg);
        // }

        if let Some(mesh) = meshes.get_mut(mesh_handle.id) {
            mesh.set_attribute(
                "Vertex_Color_FG",
                VertexAttributeValues::from(vec![
                    // top
                    [x as Real / 80.0, y as Real / 50.0, 0.0],
                    [x as Real / 80.0, y as Real / 50.0, 0.0],
                    [x as Real / 80.0, y as Real / 50.0, 0.0],
                    [x as Real / 80.0, y as Real / 50.0, 0.0],
                ]),
            );
        }

        if !map.revealed[idx] {
            sprite.color = Color::BLACK;
        } else if !map.visible[idx] {
            sprite.color = Color::GRAY;
        } else {
            sprite.color = Color::WHITE;
        }
    }
    println!("Done drawing");
}

pub fn load_char_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_query: Query<&AreaGrid>,
    window: Res<WindowDescriptor>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    let map = map_query.single();

    // TODO: load as a uniform sampler 2d
    // derive tile from cp437 index
    let texture_handle = asset_server.load("16x16-RogueYun-AgmEdit.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut children: Vec<Entity> = vec![];

    let pipeline = tile_shader_pipeline(pipelines, shaders);

    for index in 0..map.tiles.len() {
        let mut quad_mesh = Mesh::from(shape::Quad::new(RealPos::new(
            TILE_SIZE as Real,
            TILE_SIZE as Real,
        )));
        quad_mesh.set_attribute(
            // name of the attribute
            "Vertex_Color_FG",
            // the vertex attributes, represented by `VertexAttributeValues`
            // NOTE: the attribute count has to be consistent across all attributes, otherwise bevy
            // will panic.
            VertexAttributeValues::from(vec![
                // top
                [0.79, 0.73, 0.07],
                [0.74, 0.14, 0.29],
                [0.08, 0.55, 0.74],
                [0.20, 0.27, 0.29],
            ]),
        );
        let quad_handle = meshes.add(quad_mesh);

        let (x, y) = map.idx_xy(index);
        let pos = RealPos::new(
            -window.width / 2.0 + (x * TILE_SIZE + TILE_SIZE / 2) as Real,
            window.height / 2.0 - (y * TILE_SIZE + TILE_SIZE / 2) as Real,
        );
        children.push(spawn_sprite_tile(
            &mut commands,
            pos,
            texture_atlas_handle.clone(),
            quad_handle.clone(),
            pipeline.clone(),
        ));
    }

    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Grid)
        .insert_children(0, &children);
}

fn tile_shader_pipeline(
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
) -> Handle<PipelineDescriptor> {
    // Create a new shader pipeline.
    pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }))
}

fn spawn_sprite_tile(
    commands: &mut Commands,
    pos: RealPos,
    texture_atlas_handle: Handle<TextureAtlas>,
    mesh: Handle<Mesh>,
    pipeline: Handle<PipelineDescriptor>,
) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: pos.extend(0.0),
                ..Default::default()
            },
            mesh,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline)]),
            ..Default::default()
        })
        .id()
}
