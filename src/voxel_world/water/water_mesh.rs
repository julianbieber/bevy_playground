use crate::voxel_world::voxel::{Voxel, VoxelDirection, VoxelPosition};
use crate::voxel_world::{access::VoxelAccess, voxel::HALF_VOXEL_SIZE};
use bevy::{math::Vec3, render::pipeline::PrimitiveTopology};
use bevy::{
    prelude::Mesh,
    render::mesh::{Indices, VertexAttributeValues},
};
use std::borrow::Cow;

use super::water::{Water, WaterVoxel, WATER_QUADS};

pub(super) const UNUSED: f32 = 100000000.0;

pub(super) const VERTEX_BUFFER_SIZE: usize = 4 * WATER_QUADS;

impl Water {
    pub fn initial_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices = vec![[UNUSED, UNUSED, UNUSED]; VERTEX_BUFFER_SIZE];
        let normals = vec![[UNUSED, UNUSED, UNUSED]; VERTEX_BUFFER_SIZE];
        let fill_buffer = vec![1.0f32; VERTEX_BUFFER_SIZE];
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
        mesh.set_attribute("Water_Fill", fill_buffer);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    pub fn update_mesh(&mut self, mut mesh: &mut Mesh, voxel_access: &VoxelAccess) {
        let needs_remesh = self.apply_changes();
        let mut vertices = if let VertexAttributeValues::Float32x3(vertices) = mesh
            .attribute_mut(Cow::Borrowed(Mesh::ATTRIBUTE_POSITION))
            .unwrap()
        {
            vertices
        } else {
            panic!("vertices in wrong format");
        };

        for unused in self
            .voxels
            .iter()
            .filter(|(_, v)| v.fill < 0.00001)
            .flat_map(|(_, v)| v.indices.iter())
        {
            for i in unused.iter() {
                vertices[(*i) as usize] = [UNUSED, UNUSED, UNUSED];
            }

            self.unused.push_back(*unused);
        }
        self.voxels.retain(|_, v| v.fill >= 0.00001);

        let mut top_indices: Vec<(u32, f32)> = Vec::with_capacity(128);
        let mut bottom_indices: Vec<(u32, f32)> = Vec::with_capacity(128);
        let mut left_indices: Vec<(u32, f32)> = Vec::with_capacity(128);
        let mut right_indices: Vec<(u32, f32)> = Vec::with_capacity(128);
        let mut front_indices: Vec<(u32, f32)> = Vec::with_capacity(128);
        let mut back_indices: Vec<(u32, f32)> = Vec::with_capacity(128);

        {
            for changed in needs_remesh.iter() {
                if let Some(water_voxel) = self.voxels.get(changed) {
                    for is in water_voxel.indices.iter() {
                        for i in is.iter() {
                            vertices[(*i) as usize] = [UNUSED, UNUSED, UNUSED];
                        }

                        self.unused.push_back(*is);
                    }
                }
            }

            for added in needs_remesh.iter() {
                if self.voxels.get(added).is_some() {
                    let fill = self.get_fill(added);
                    let mut voxel_indices = Vec::new();
                    self.set_top_plane_vertices(*added, fill, vertices, voxel_access)
                        .map(|v| {
                            v.iter().for_each(|s| {
                                top_indices.push((*s, fill));
                            });
                            voxel_indices.push(v);
                        });
                    self.set_bottom_plane_vertices(*added, vertices, voxel_access)
                        .map(|v| {
                            v.iter().for_each(|s| {
                                bottom_indices.push((*s, fill));
                            });
                            voxel_indices.push(v);
                        });
                    self.set_plane_vertices(
                        *added,
                        fill,
                        VoxelDirection::LEFT,
                        set_left_vertices,
                        vertices,
                        voxel_access,
                    )
                    .map(|v| {
                        v.iter().for_each(|s| {
                            left_indices.push((*s, fill));
                        });
                        voxel_indices.push(v);
                    });
                    self.set_plane_vertices(
                        *added,
                        fill,
                        VoxelDirection::RIGHT,
                        set_right_vertices,
                        vertices,
                        voxel_access,
                    )
                    .map(|v| {
                        v.iter().for_each(|s| {
                            right_indices.push((*s, fill));
                        });
                        voxel_indices.push(v);
                    });
                    self.set_plane_vertices(
                        *added,
                        fill,
                        VoxelDirection::FRONT,
                        set_front_vertices,
                        vertices,
                        voxel_access,
                    )
                    .map(|v| {
                        v.iter().for_each(|s| {
                            front_indices.push((*s, fill));
                        });
                        voxel_indices.push(v);
                    });
                    self.set_plane_vertices(
                        *added,
                        fill,
                        VoxelDirection::BACK,
                        set_back_vertices,
                        vertices,
                        voxel_access,
                    )
                    .map(|v| {
                        v.iter().for_each(|s| {
                            back_indices.push((*s, fill));
                        });
                        voxel_indices.push(v);
                    });
                    self.voxels.get_mut(added).unwrap().indices = voxel_indices;
                }
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

            for (i, _) in top_indices.iter() {
                normals[*i as usize] = [0.0, 1.0, 0.0];
            }
            for (i, _) in bottom_indices.iter() {
                normals[*i as usize] = [0.0, -1.0, 0.0];
            }
            for (i, _) in left_indices.iter() {
                normals[*i as usize] = [-1.0, 0.0, 0.0];
            }
            for (i, _) in right_indices.iter() {
                normals[*i as usize] = [1.0, 0.0, 0.0];
            }
            for (i, _) in front_indices.iter() {
                normals[*i as usize] = [0.0, 0.0, -1.0];
            }
            for (i, _) in back_indices.iter() {
                normals[*i as usize] = [0.0, 0.0, 1.0];
            }
        }

        {
            let mut fill_state = if let VertexAttributeValues::Float32(vertices) =
                mesh.attribute_mut("Water_Fill").unwrap()
            {
                vertices
            } else {
                panic!("normals in wrong format");
            };

            for (i, f) in top_indices {
                fill_state[i as usize] = f;
            }
            for (i, _) in bottom_indices {
                fill_state[i as usize] = 1.0;
            }
            for c in left_indices.chunks(4) {
                fill_state[c[0].0 as usize] = 1.0;
                fill_state[c[1].0 as usize] = 1.0;
                fill_state[c[2].0 as usize] = c[2].1;
                fill_state[c[3].0 as usize] = c[3].1;
            }

            for c in right_indices.chunks(4) {
                fill_state[c[0].0 as usize] = c[0].1;
                fill_state[c[1].0 as usize] = c[1].1;
                fill_state[c[2].0 as usize] = 1.0;
                fill_state[c[3].0 as usize] = 1.0;
            }

            for c in front_indices.chunks(4) {
                fill_state[c[0].0 as usize] = 1.0;
                fill_state[c[1].0 as usize] = c[1].1;
                fill_state[c[2].0 as usize] = c[2].1;
                fill_state[c[3].0 as usize] = 1.0;
            }

            for c in back_indices.chunks(4) {
                fill_state[c[0].0 as usize] = 1.0;
                fill_state[c[1].0 as usize] = c[1].1;
                fill_state[c[2].0 as usize] = c[2].1;
                fill_state[c[3].0 as usize] = 1.0;
            }
        }
    }

    fn set_plane_vertices<F>(
        &mut self,
        voxel_position: VoxelPosition,
        fill: f32,
        plane_direction: VoxelDirection,
        mut set_fn: F,
        vertices: &mut Vec<[f32; 3]>,
        voxel_access: &VoxelAccess,
    ) -> Option<[u32; 4]>
    where
        F: FnMut(&mut Vec<[f32; 3]>, &[u32; 4], Vec3) -> (),
    {
        let in_direction = voxel_position.in_direction(plane_direction);
        if self.get_fill(&in_direction) < fill && voxel_access.get_voxel(in_direction).is_none() {
            let indices = self.unused.pop_back().unwrap();
            set_fn(vertices, &indices, voxel_position.to_vec());
            Some(indices)
        } else {
            None
        }
    }

    fn set_top_plane_vertices(
        &mut self,
        voxel_position: VoxelPosition,
        fill: f32,
        vertices: &mut Vec<[f32; 3]>,
        voxel_access: &VoxelAccess,
    ) -> Option<[u32; 4]> {
        let in_direction = voxel_position.in_direction(VoxelDirection::UP);
        if fill < 0.9
            || (self.get_fill(&in_direction) < 0.0001
                && voxel_access.get_voxel(in_direction).is_none())
        {
            let indices = self.unused.pop_back().unwrap();
            set_top_vertices(vertices, &indices, voxel_position.to_vec());
            Some(indices)
        } else {
            None
        }
    }

    fn set_bottom_plane_vertices(
        &mut self,
        voxel_position: VoxelPosition,
        vertices: &mut Vec<[f32; 3]>,
        voxel_access: &VoxelAccess,
    ) -> Option<[u32; 4]> {
        let in_direction = voxel_position.in_direction(VoxelDirection::DOWN);
        if self.get_fill(&in_direction) <= 0.9 && voxel_access.get_voxel(in_direction).is_none() {
            let indices = self.unused.pop_back().unwrap();
            set_bottom_vertices(vertices, &indices, voxel_position.to_vec());
            Some(indices)
        } else {
            None
        }
    }

    fn get_fill(&self, voxel_position: &VoxelPosition) -> f32 {
        self.voxels
            .get(voxel_position)
            .map(|v| v.fill)
            .unwrap_or(0.0)
    }
}

fn set_top_vertices(vertices: &mut Vec<[f32; 3]>, indices: &[u32; 4], center: Vec3) {
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
}

fn set_bottom_vertices(vertices: &mut Vec<[f32; 3]>, indices: &[u32; 4], center: Vec3) {
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
}

fn set_left_vertices(vertices: &mut Vec<[f32; 3]>, indices: &[u32; 4], center: Vec3) {
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
}

fn set_right_vertices(vertices: &mut Vec<[f32; 3]>, indices: &[u32; 4], center: Vec3) {
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
}

fn set_front_vertices(vertices: &mut Vec<[f32; 3]>, indices: &[u32; 4], center: Vec3) {
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
}

fn set_back_vertices(vertices: &mut Vec<[f32; 3]>, indices: &[u32; 4], center: Vec3) {
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
}
