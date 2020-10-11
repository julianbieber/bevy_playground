pub const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 1, binding = 1) uniform CustomMaterial_time {
    float time;
};
void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position.x, Vertex_Position.y + sin(time * 0.1 + Vertex_Position.y), Vertex_Position.z, 1.0);
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
layout(set = 1, binding = 1) uniform CustomMaterial_time {
    float time;
};
void main() {
    o_Target = vec4(0, 0, (sin(gl_FragCoord.x + gl_FragCoord.y + time) + 1) / 2 * 255, 1.0);
}
"#;
