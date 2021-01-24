use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use rand::{thread_rng, Rng};

use crate::particles::model::{ParticleDescription, ParticleTypes};
use crate::particles::primitives::{cube_indices, cube_vertices};

pub fn create_particle_mesh(particles: &ParticleDescription) -> Mesh {
    let (positions, indices, particle_directions) = match particles.typ {
        ParticleTypes::Explosion { radius } => {
            let cube_vertices = cube_vertices(0.02);
            let mut positions = Vec::with_capacity(24 * particles.particles as usize);
            let mut indices = Vec::with_capacity(36 * particles.particles as usize);
            let mut particle_directions = Vec::with_capacity(24 * particles.particles as usize);
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
            (positions, indices, particle_directions)
        }
    };

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh.set_attribute(
        "Particle_Direction",
        VertexAttributeValues::from(particle_directions),
    );
    mesh
}
