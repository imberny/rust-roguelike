use bevy::{
    core::Bytes,
    prelude::*,
    render::renderer::{RenderResource, RenderResourceType, RenderResources},
};

#[derive(Debug, Clone, RenderResources, Component)]
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
