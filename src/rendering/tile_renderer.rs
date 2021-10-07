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

use super::{FRAGMENT_SHADER, VERTEX_SHADER};

const TILE_SIZE: Int = 16;

pub struct Grid;

pub fn draw(
    mut meshes: ResMut<Assets<Mesh>>,
    map_query: Query<&AreaGrid, Changed<AreaGrid>>,
    mut query: Query<&Children, With<Grid>>,
    mut tile_query: Query<(&mut Handle<Mesh>, &mut Sprite)>,
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

        if !map.revealed[idx] {
            fg = Color::BLACK;
            bg = Color::BLACK;
        } else if !map.visible[idx] {
            fg = greyscale(&fg);
            bg = greyscale(&bg);
        }

        if let Some(mesh) = meshes.get_mut(mesh_handle.id) {
            mesh.set_attribute(
                "Vertex_Color_Foreground",
                VertexAttributeValues::from(vec![
                    fg.as_linear_rgba_f32(),
                    fg.as_linear_rgba_f32(),
                    fg.as_linear_rgba_f32(),
                    fg.as_linear_rgba_f32(),
                    // [x as Real / 80.0, y as Real / 50.0, 0.0],
                    // [x as Real / 80.0, y as Real / 50.0, 0.0],
                    // [x as Real / 80.0, y as Real / 50.0, 0.0],
                    // [x as Real / 80.0, y as Real / 50.0, 0.0],
                ]),
            );
            mesh.set_attribute(
                "Vertex_Color_Background",
                VertexAttributeValues::from(vec![
                    bg.as_linear_rgba_f32(),
                    bg.as_linear_rgba_f32(),
                    bg.as_linear_rgba_f32(),
                    bg.as_linear_rgba_f32(),
                    // [0.0, 0.0, 0.1 + x as Real / 360.0, 1.0],
                    // [0.0, 0.0, 0.1 + x as Real / 360.0, 1.0],
                    // [0.0, 0.0, 0.1 + x as Real / 360.0, 1.0],
                    // [0.0, 0.0, 0.1 + x as Real / 360.0, 1.0],
                ]),
            );
            mesh.set_attribute(
                "Vertex_CP437_Index",
                VertexAttributeValues::Uint32(vec![index, index, index, index]),
            );
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

struct LoadingTexture(Option<Handle<Texture>>);

struct CP437Pipeline(Handle<PipelineDescriptor>);

pub fn load_char_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_query: Query<&AreaGrid>,
    window: Res<WindowDescriptor>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,

    mut cp437_assets: ResMut<Assets<CP437TilesetTexture>>,
) {
    let map = map_query.single();

    // TODO: load as a uniform sampler 2d
    // derive tile from cp437 index
    // let texture_handle = asset_server.load("16x16-RogueYun-AgmEdit.png");
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut children: Vec<Entity> = vec![];

    // Create a new shader pipeline.
    let pipeline = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    // commands.insert_resource(CP437Pipeline(pipeline));

    // Start loading the texture.
    // commands.insert_resource(LoadingTexture(Some(
    // let cp_tileset: Handle<Texture> = asset_server.load("16x16-RogueYun-AgmEdit.png");
    // )));

    let cp437_handle = cp437_assets.add(CP437TilesetTexture {
        texture: asset_server.load("16x16-RogueYun-AgmEdit.png"),
    });

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind CP437TilesetTexture resources
    // to our shader.
    render_graph.add_system_node(
        "cp437_tileset_texture",
        AssetRenderResourcesNode::<CP437TilesetTexture>::new(false),
    );
    // Add a Render Graph edge connecting our new "my_array_texture" node to the main pass node.
    // This ensures "cp437_tileset_texture" runs before the main pass.
    render_graph
        .add_node_edge("cp437_tileset_texture", base::node::MAIN_PASS)
        .unwrap();

    for index in 0..map.tiles.len() {
        let mut quad_mesh = Mesh::from(shape::Quad::new(RealPos::new(
            TILE_SIZE as Real,
            TILE_SIZE as Real,
        )));
        quad_mesh.set_attribute(
            "Vertex_Color_Foreground",
            VertexAttributeValues::from(vec![
                // top
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ]),
        );
        quad_mesh.set_attribute(
            "Vertex_Color_Background",
            VertexAttributeValues::from(vec![
                // top
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ]),
        );
        quad_mesh.set_attribute(
            "Vertex_CP437_Index",
            VertexAttributeValues::Uint32(vec![3, 3, 3, 3]),
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
            quad_handle.clone(),
            pipeline.clone(),
            cp437_handle.clone(),
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
    mesh: Handle<Mesh>,
    pipeline: Handle<PipelineDescriptor>,
    cp437: Handle<CP437TilesetTexture>,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: pos.extend(0.0),
                ..Default::default()
            },
            mesh,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline)]),
            ..Default::default()
        })
        .insert(cp437)
        .id()
}
