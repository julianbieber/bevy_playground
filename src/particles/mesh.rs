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

use crate::particles::model::Explosion;
use crate::particles::primitives::{cube_indices, cube_vertices};
use flume::{unbounded, Receiver, Sender};
use std::time::Duration;

pub fn create_explosion_mesh(explosion: &Explosion) -> Mesh {
    let particle_count = explosion.particles;
    let radius = explosion.radius;
    let cube_vertices = cube_vertices(0.02);
    let mut positions = Vec::with_capacity(24 * particle_count as usize);
    let mut indices = Vec::with_capacity(36 * particle_count as usize);
    let mut particle_directions = Vec::with_capacity(24 * particle_count as usize);
    for i in 0..particle_count {
        positions.extend_from_slice(&cube_vertices);
        indices.extend(cube_indices(i).iter());
    }
    for _ in 0..particle_count {
        let mut x: f32 = thread_rng().gen_range(-radius, radius);
        let mut y: f32 = thread_rng().gen_range(-radius, radius);
        let mut z: f32 = thread_rng().gen_range(-radius, radius);
        let div = (x * x + y * y + z * z).sqrt();
        x /= div;
        y /= div;
        z /= div;
        let d = thread_rng().gen_range(0.0, 1.0);

        for _ in 0..8 {
            particle_directions.push([x * d, y * d, z * d]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh.set_attribute(
        "Particle_Direction",
        VertexAttributeValues::from(particle_directions),
    );
    mesh
}
