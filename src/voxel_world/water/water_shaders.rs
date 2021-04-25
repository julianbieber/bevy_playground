pub const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 2, binding = 0) uniform WaterMaterial_time {
    float time;
};
layout(location = 1) out X_Out {
    float x;
}x_out;
void main() {
    vec4 world_position = vec4(Vertex_Position, 1);
    world_position.y = world_position.y + sin(time * 0.5 + world_position.x) * 0.2;
    gl_Position = ViewProj * world_position;
    x_out.x = world_position.x;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform WaterMaterial_time {
    float time;
};
layout(location = 1) in float x;
void main() {
    o_Target = vec4(0, 0, 1, abs(sin(x + time)));
}
"#;
