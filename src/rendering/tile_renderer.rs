use bevy::{
    core::Bytes,
    prelude::*,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{RenderGraph, RenderResourcesNode},
        renderer::{RenderResource, RenderResourceType, RenderResources},
        shader::{ShaderStage, ShaderStages},
    },
};

use crate::{
    core::types::{GridPos, Int, Real, RealPos},
    util::helpers::colors::greyscale,
    world::{AreaGrid, TileType},
};

const TILE_SIZE: Int = 16;

pub mod node {
    pub const CP437_TILE: &str = "cp437_tile";
}

pub struct Grid;

pub fn draw(
    map_query: Query<&AreaGrid, Changed<AreaGrid>>,
    mut query: Query<&Children, With<Grid>>,
    mut tile_query: Query<(&mut TextureAtlasSprite, &mut CP437Tile)>,
) {
    if map_query.is_empty() {
        return;
    }
    let map = map_query.single();

    let children = query.single_mut();

    assert!(!map.tiles.is_empty());
    for (idx, tile) in map.tiles.iter().enumerate() {
        let tile_entity = children[idx];
        let (mut sprite, mut cp_tile) = tile_query.get_mut(tile_entity).unwrap();

        let (x, y) = map.idx_xy(idx);
        let pos = GridPos::new(x, y);

        let mut index = match tile {
            TileType::Wall => 35_u32,
            TileType::Floor => 46_u32,
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

        cp_tile.fg = fg;
        cp_tile.bg = bg;

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

#[derive(Debug, Clone, RenderResources)]
#[render_resources(from_self)]
#[repr(C)]
pub struct CP437Tile {
    pub fg: Color,
    pub bg: Color,
}

impl RenderResource for CP437Tile {
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Buffer)
    }

    fn buffer_byte_len(&self) -> Option<usize> {
        Some(32)
    }

    fn write_buffer_bytes(&self, buffer: &mut [u8]) {
        // Write the color buffer
        let (color_buf, rest) = buffer.split_at_mut(16);
        self.fg.write_bytes(color_buf);
        self.bg.write_bytes(rest);
    }

    fn texture(&self) -> Option<&Handle<Texture>> {
        None
    }
}

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
            include_str!("shaders/cp437.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("shaders/cp437.frag"),
        ))),
    }));

    render_graph.add_system_node(
        node::CP437_TILE,
        RenderResourcesNode::<CP437Tile>::new(false),
    );

    for index in 0..map.tiles.len() {
        let (x, y) = map.idx_xy(index);
        let pos = RealPos::new(
            -window.width / 2.0 + (x * TILE_SIZE + TILE_SIZE / 2) as Real,
            window.height / 2.0 - (y * TILE_SIZE + TILE_SIZE / 2) as Real,
        );
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
