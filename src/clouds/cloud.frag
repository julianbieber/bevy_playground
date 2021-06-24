#version 450

layout(location = 0) in vec3 v_WorldPosition;
layout(location = 1) in vec3 v_WorldNormal;
layout(location = 2) in vec3 v_Uv;
layout(location = 3) in vec3 v_viewDir;
layout(location = 0) out vec4 o_Target;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(std140, set = 3, binding = 1) uniform CameraPosition {
    vec4 CameraPos;
};
layout(set = 2, binding = 0) uniform texture3D CloudMaterial_color;
layout(set = 2, binding = 1) uniform sampler CloudMaterial_color_sampler;

void main() {
    float i = 0.0;
    float step_size =  1.0 / 128.0;
    vec4 inversed_intensity = vec4(0.0, 0.0, 0.0, 0.0);
    vec3 sample_position = (- i * v_viewDir + v_Uv);
    while (i < 1.0){
        inversed_intensity = inversed_intensity + texture(sampler3D(CloudMaterial_color, CloudMaterial_color_sampler), sample_position)*step_size;
        i = i + step_size;
        sample_position = (- i * v_viewDir + v_Uv);
    }
    float intensity = 1.0 - inversed_intensity.x + 1.0 - inversed_intensity.y + 1.0 - inversed_intensity.z + 1.0 - inversed_intensity.w;
    o_Target =  vec4(intensity, intensity, intensity, intensity);
}

