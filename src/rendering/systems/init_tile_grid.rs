use bevy::{
    prelude::*,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{RenderGraph, RenderResourcesNode},
        shader::{ShaderStage, ShaderStages},
    },
};

use crate::{
    core::types::{Int, Real},
    rendering::{
        constants::{CP437_TILE_RENDER_NODE, TILE_SIZE, WORLD_VIEWPORT_DIMENSIONS},
        CP437Tile, Grid,
    },
};

pub fn load_char_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<WindowDescriptor>,

    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,

    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("16x16-RogueYun-AgmEdit.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut children: Vec<Entity> = vec![];

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../shaders/cp437.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../shaders/cp437.frag"),
        ))),
    }));

    render_graph.add_system_node(
        CP437_TILE_RENDER_NODE,
        RenderResourcesNode::<CP437Tile>::new(false),
    );

    let (columns, rows) = WORLD_VIEWPORT_DIMENSIONS;
    (0..columns).for_each(|column| {
        (0..rows).for_each(|row| {
            let pos = Vec2::new(
                -window.width / 2.0 + (column * TILE_SIZE + TILE_SIZE / 2) as Real,
                window.height / 2.0 - (row * TILE_SIZE + TILE_SIZE / 2) as Real,
            );
            let sprite_tile = commands
                .spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        translation: pos.extend(0.0),
                        ..Default::default()
                    },
                    render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                        pipeline_handle.clone(),
                    )]),
                    texture_atlas: texture_atlas_handle.clone(),
                    ..Default::default()
                })
                .insert(CP437Tile {
                    fg: Color::GOLD,
                    bg: Color::DARK_GRAY,
                })
                .id();
            children.push(sprite_tile);
        });
    });

    commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Grid)
        .insert_children(0, &children);
}
