pub const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Color_FG;
layout(location = 0) out vec3 v_color_fg;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    v_color_fg = Vertex_Color_FG;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 v_color_fg;
layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = vec4(v_color_fg, 1.0);
}
"#;
