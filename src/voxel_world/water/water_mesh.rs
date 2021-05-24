use crate::voxel_world::voxel::HALF_VOXEL_SIZE;
use ahash::AHashSet;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::{
    prelude::Mesh,
    render::mesh::{Indices, VertexAttributeValues},
};
use std::borrow::Cow;

use super::water::{Water, WaterOperations, WaterVoxel, WATER_QUADS};

pub(super) const UNUSED: f32 = 100000000.0;

pub(super) const VERTEX_BUFFER_SIZE: usize = 4 * WATER_QUADS;

impl Water {
    pub fn initial_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices = vec![[UNUSED, UNUSED, UNUSED]; VERTEX_BUFFER_SIZE];
        let normals = vec![[UNUSED, UNUSED, UNUSED]; VERTEX_BUFFER_SIZE];
        let uvs = vec![[0.0, 0.0]; VERTEX_BUFFER_SIZE];
        let mut indices: Vec<u32> = Vec::with_capacity(WATER_QUADS * 6);
        for i in 0..WATER_QUADS {
            indices.push(i as u32 * 4 + 0);
            indices.push(i as u32 * 4 + 1);
            indices.push(i as u32 * 4 + 2);
            indices.push(i as u32 * 4 + 0);
            indices.push(i as u32 * 4 + 2);
            indices.push(i as u32 * 4 + 3);
        }
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_POSITION), vertices);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_NORMAL), normals);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_UV_0), uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    pub fn update_mesh(&mut self, mut mesh: &mut Mesh, mut water_operations: &mut WaterOperations) {
        let mut actually_added = AHashSet::new();
        for a in water_operations.added.iter() {
            if !self.voxels.contains_key(a) {
                actually_added.insert(a.clone());
            }
        }

        let mut vertices = if let VertexAttributeValues::Float32x3(vertices) = mesh
            .attribute_mut(Cow::Borrowed(Mesh::ATTRIBUTE_POSITION))
            .unwrap()
        {
            vertices
        } else {
            panic!("vertices in wrong format");
        };

        for removed in water_operations.removed.iter() {
            if let Some(newly_unused) = self.voxels.remove(removed) {
                for n in newly_unused.indices.iter() {
                    vertices[n[0] as usize] = [UNUSED, UNUSED, UNUSED];
                    vertices[n[1] as usize] = [UNUSED, UNUSED, UNUSED];
                    vertices[n[2] as usize] = [UNUSED, UNUSED, UNUSED];
                    vertices[n[3] as usize] = [UNUSED, UNUSED, UNUSED];
                    self.unused.push_back(n.clone());
                }
            }
        }

        let mut top_indices: Vec<u32> = Vec::with_capacity(128);
        let mut bottom_indices: Vec<u32> = Vec::with_capacity(128);
        let mut left_indices: Vec<u32> = Vec::with_capacity(128);
        let mut right_indices: Vec<u32> = Vec::with_capacity(128);
        let mut front_indices: Vec<u32> = Vec::with_capacity(128);
        let mut back_indices: Vec<u32> = Vec::with_capacity(128);

        {
            let mut vertices = if let VertexAttributeValues::Float32x3(vertices) = mesh
                .attribute_mut(Cow::Borrowed(Mesh::ATTRIBUTE_POSITION))
                .unwrap()
            {
                vertices
            } else {
                panic!("vertices in wrong format");
            };

            for removed in water_operations.removed.iter() {
                if let Some(water_voxel) = self.voxels.get(removed) {
                    for is in water_voxel.indices.iter() {
                        for i in is.iter() {
                            vertices[(*i) as usize] = [UNUSED, UNUSED, UNUSED];
                        }

                        self.unused.push_back(*is);
                    }
                }
            }

            for added in actually_added.iter() {
                let center = added.to_vec();
                let mut all_indices = Vec::new();
                // TOP
                let indices = self.unused.pop_back().unwrap();
                all_indices.push(indices.clone());
                vertices[indices[0] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                vertices[indices[1] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[2] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[3] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                for i in indices.iter() {
                    top_indices.push(*i)
                }
                // BOTTOM
                let indices = self.unused.pop_back().unwrap();
                all_indices.push(indices.clone());
                vertices[indices[3] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                vertices[indices[2] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[1] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[0] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                for i in indices.iter() {
                    bottom_indices.push(*i)
                }
                // LEFT
                let indices = self.unused.pop_back().unwrap();
                all_indices.push(indices.clone());
                vertices[indices[0] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                vertices[indices[1] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[2] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[3] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                for i in indices.iter() {
                    left_indices.push(*i)
                }
                // RIGHT
                let indices = self.unused.pop_back().unwrap();
                all_indices.push(indices.clone());
                vertices[indices[3] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                vertices[indices[2] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[1] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[0] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                for i in indices.iter() {
                    right_indices.push(*i)
                }
                // FRONT
                let indices = self.unused.pop_back().unwrap();
                all_indices.push(indices.clone());
                vertices[indices[0] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                vertices[indices[1] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                vertices[indices[2] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                vertices[indices[3] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z - HALF_VOXEL_SIZE,
                ];
                for i in indices.iter() {
                    front_indices.push(*i)
                }
                // BACK
                let indices = self.unused.pop_back().unwrap();
                all_indices.push(indices.clone());
                vertices[indices[3] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[2] as usize] = [
                    center.x - HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[1] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y + HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                vertices[indices[0] as usize] = [
                    center.x + HALF_VOXEL_SIZE,
                    center.y - HALF_VOXEL_SIZE,
                    center.z + HALF_VOXEL_SIZE,
                ];
                for i in indices.iter() {
                    back_indices.push(*i)
                }
                self.voxels.insert(
                    added.clone(),
                    WaterVoxel {
                        indices: all_indices,
                    },
                );
            }
        }
        {
            let mut normals = if let VertexAttributeValues::Float32x3(vertices) = mesh
                .attribute_mut(Cow::Borrowed(Mesh::ATTRIBUTE_NORMAL))
                .unwrap()
            {
                vertices
            } else {
                panic!("normals in wrong format");
            };

            for i in top_indices {
                normals[i as usize] = [0.0, 1.0, 0.0];
            }
            for i in bottom_indices {
                normals[i as usize] = [0.0, -1.0, 0.0];
            }
            for i in left_indices {
                normals[i as usize] = [-1.0, 0.0, 0.0];
            }
            for i in right_indices {
                normals[i as usize] = [1.0, 0.0, 0.0];
            }
            for i in front_indices {
                normals[i as usize] = [0.0, 0.0, -1.0];
            }
            for i in back_indices {
                normals[i as usize] = [0.0, 0.0, 1.0];
            }
        }

        water_operations.removed = AHashSet::new();
        water_operations.added = AHashSet::new();
    }
}
