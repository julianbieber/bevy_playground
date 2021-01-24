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

#[derive(Clone)]
pub struct Explosion {
    pub duration: Duration,
    pub radius: f32,
    pub particles: u32,
    pub position: Vec3,
}
