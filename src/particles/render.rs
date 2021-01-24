use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::VertexAttributeValues,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
    tasks::AsyncComputeTaskPool,
};

use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use rand::{thread_rng, Rng};

use flume::{unbounded, Receiver, Sender};
use std::time::Duration;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "b8ba5506-487a-4fed-88a2-c6bac4a22016"]
pub struct ParticleDirectionMaterial {
    pub multiplier: f32,
}

pub struct ParticlePipeline {
    pub handle: Handle<PipelineDescriptor>,
}

pub fn setup_particles(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    render_graph.add_system_node(
        "particle",
        AssetRenderResourcesNode::<ParticleDirectionMaterial>::new(true),
    );

    render_graph
        .add_node_edge("particle", base::node::MAIN_PASS)
        .unwrap();
    commands.insert_resource(ParticlePipeline {
        handle: pipeline_handle,
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
        m.multiplier = old + time.delta_seconds() * 10.0;
    }
}

const VERTEX_SHADER: &str = r#"
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
    gl_Position = ViewProj * Model * (vec4(Vertex_Position, 1.0) + vec4(Particle_Direction, 0.0) * multiplier);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
void main() {
    o_Target = vec4(1.0, 0.5, 0.5, 0.5);
}
"#;
