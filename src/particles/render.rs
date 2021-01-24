use crate::particles::model::ParticleTypes;
use ahash::AHashMap;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "b8ba5506-487a-4fed-88a2-c6bac4a22016"]
pub struct ParticleDirectionMaterial {
    pub multiplier: f32,
}

pub struct ParticlePipeline {
    pub explosion_handle: Handle<PipelineDescriptor>,
    pub high_storms_handle: Handle<PipelineDescriptor>,
}

pub fn setup_particles(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let explosion_pipeline_handle =
        pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                EXPLOSION_VERTEX_SHADER,
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                EXPLOSION_FRAGMENT_SHADER,
            ))),
        }));

    render_graph.add_system_node(
        "explosion",
        AssetRenderResourcesNode::<ParticleDirectionMaterial>::new(true),
    );

    render_graph
        .add_node_edge("explosion", base::node::MAIN_PASS)
        .unwrap();

    let high_storm_pipeline_handle =
        pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                HIGH_STORM_VERTEX_SHADER,
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                HIGH_STORM_FRAGMENT_SHADER,
            ))),
        }));

    render_graph.add_system_node(
        "high_storm",
        AssetRenderResourcesNode::<ParticleDirectionMaterial>::new(true),
    );

    render_graph
        .add_node_edge("high_storm", base::node::MAIN_PASS)
        .unwrap();

    commands.insert_resource(ParticlePipeline {
        explosion_handle: explosion_pipeline_handle,
        high_storms_handle: high_storm_pipeline_handle,
    });
}

pub fn update_particle_direction(
    mut material: ResMut<Assets<ParticleDirectionMaterial>>,
    time: Res<Time>,
) {
    let handles: Vec<_> = material.ids().collect();
    for handle in handles.into_iter() {
        let m = material.get_mut(handle).unwrap();
        let old = m.multiplier;
        m.multiplier = old + time.delta_seconds();
    }
}

const EXPLOSION_VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Particle_Direction;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 2, binding = 0) uniform ParticleDirectionMaterial_multiplier {
    float multiplier;
};
void main() {
    vec3 offset = Particle_Direction * multiplier;
    gl_Position = ViewProj * Model * (vec4(Vertex_Position, 1.0) + vec4(offset, 0.0));
}
"#;

const EXPLOSION_FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
void main() {
    o_Target = vec4(1.0, 0.5, 0.5, 0.5);
}
"#;

const HIGH_STORM_VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Particle_Direction;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 2, binding = 0) uniform ParticleDirectionMaterial_multiplier {
    float multiplier;
};
void main() {
    vec3 offset = Particle_Direction * multiplier;
    offset.x = mod(offset.x, 10.0);
    gl_Position = ViewProj * Model * (vec4(Vertex_Position, 1.0) + vec4(offset, 0.0));
}
"#;

const HIGH_STORM_FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
void main() {
    o_Target = vec4(1.0, 1.0, 1.0, 0.9);
}
"#;
