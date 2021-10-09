use bevy::{
    prelude::*,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{RenderGraph, RenderResourcesNode},
        shader::{ShaderStage, ShaderStages},
    },
};

use crate::{
    core::types::{Real, RealPos},
    rendering::{
        constants::{CP437_TILE_RENDER_NODE, TILE_SIZE},
        CP437Tile, Grid,
    },
    world::AreaGrid,
};

pub fn load_char_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_query: Query<&AreaGrid>,
    window: Res<WindowDescriptor>,

    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,

    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map = map_query.single();

    let texture_handle = asset_server.load("16x16-RogueYun-AgmEdit.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut children: Vec<Entity> = vec![];

    let pipeline = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
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

    for index in 0..map.tiles.len() {
        let (x, y) = map.idx_xy(index);
        let pos = RealPos(Vec2::new(
            -window.width / 2.0 + (x * TILE_SIZE + TILE_SIZE / 2) as Real,
            window.height / 2.0 - (y * TILE_SIZE + TILE_SIZE / 2) as Real,
        ));
        children.push(spawn_sprite_tile(
            &mut commands,
            pos,
            pipeline.clone(),
            texture_atlas_handle.clone(),
            CP437Tile {
                fg: Color::GOLD,
                bg: Color::DARK_GRAY,
            },
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
    cp_437: CP437Tile,
) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: pos.0.extend(0.0),
                ..Default::default()
            },
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline)]),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(cp_437)
        .id()
}
