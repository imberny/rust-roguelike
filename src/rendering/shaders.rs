pub const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 3) in uint Vertex_CP437_Index;
layout(location = 4) in vec4 Vertex_Color_Foreground;
layout(location = 5) in vec4 Vertex_Color_Background;

layout(location = 0) out vec2 v_uv;
layout(location = 1) out vec4 color_foreground;
layout(location = 2) out vec4 color_background;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    color_foreground = Vertex_Color_Foreground;
    color_background = Vertex_Color_Background;
    int index = int(Vertex_CP437_Index);

    ivec2 tile_count = ivec2(16, 16);
    // Get the position of the tile in the sprite sheet
    int y = index / tile_count.x;
    ivec2 tile_pos = ivec2(
        index - y * tile_count.x,
      y
    );

    // Adjust the uv to select the correct portion of the tileset
    v_uv = Vertex_Uv / vec2(tile_count) + 1.0 / vec2(tile_count) * vec2(tile_pos);
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec2 v_uv;
layout(location = 1) in vec4 color_foreground;
layout(location = 2) in vec4 color_background;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform texture2D CP437TilesetTexture_texture;
layout(set = 2, binding = 1) uniform sampler CP437TilesetTexture_texture_sampler;

void main() {
    vec4 color = texture(sampler2D(CP437TilesetTexture_texture, CP437TilesetTexture_texture_sampler), v_uv);

    if (color == vec4(1.0)) {
        color = color_foreground;
    } else {
        color = color_background;
    }

    o_Target = color;
}
"#;
