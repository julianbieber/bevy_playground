pub const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 1, binding = 1) uniform WaterMaterial_time {
    float time;
};
void main() {
    vec4 world_position = Model * vec4(Vertex_Position, 1);
    world_position.y = world_position.y + sin(time * 0.1 + world_position.x) + sin(world_position.z + 0.5);
    gl_Position = ViewProj * world_position;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
layout(set = 1, binding = 1) uniform WaterMaterial_time {
    float time;
};
void main() {
    o_Target = vec4(0, 0, 255, 1.0);
}
"#;
