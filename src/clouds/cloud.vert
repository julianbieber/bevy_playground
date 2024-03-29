#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;
layout(location = 0) out vec3 v_WorldPosition;
layout(location = 1) out vec3 v_WorldNormal;
layout(location = 2) out vec2 v_Uv;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(std140, set = 3, binding = 1) uniform CameraPosition {
    vec4 CameraPos;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    v_Uv = Vertex_Uv;
    vec4 world_position = Model * vec4(Vertex_Position, 1.0);
    v_WorldPosition = world_position.xyz;
    v_WorldNormal = mat3(Model) * Vertex_Normal;
    vec3 viewd = vec3( CameraPos.x,CameraPos.y,CameraPos.z) ;
    gl_Position = ViewProj * world_position;
}