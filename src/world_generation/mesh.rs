use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use std::borrow::Cow;

use super::voxel::{VoxelTypes, HALF_VOXEL_SIZE};
use super::world_structure::Terrain;

impl From<&Terrain> for Mesh {
    fn from(terrain: &Terrain) -> Self {
        let voxels = &terrain.structure;
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut draw_c = 0;

        for (_, x_map) in voxels.iter() {
            for (_, z_map) in x_map.iter() {
                for (_, voxel) in z_map.iter() {
                    draw_c += 1;
                    let voxel_world_position = voxel.position.to_vec();
                    let x = voxel_world_position.x();
                    let y = voxel_world_position.y();
                    let z = voxel_world_position.z();
                    let (u_min, u_max, v_min, v_max) = match voxel.typ {
                        VoxelTypes::DarkRock1 => (0.0f32, 0.25f32, 0.5f32, 1.0f32),
                        VoxelTypes::DarkRock2 => (0.25f32, 0.5f32, 0.5f32, 1.0f32),
                        VoxelTypes::Lava => (0.25, 0.5, 0.0, 0.5),
                        VoxelTypes::Moss => (0.5, 0.75, 0.0, 0.5),
                        VoxelTypes::LightRock1 => (0.75, 1.0, 0.0, 0.5),
                        VoxelTypes::LightRock2 => (0.5, 0.75, 0.5, 1.0),
                        VoxelTypes::CrackedRock => (0.0, 0.25, 0.0, 0.5),
                    };

                    let v = &[
                        // top (0., 0., size)
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [0., 0., 1.0f32],
                            [u_min, v_min],
                        ),
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [0., 0., 1.0f32],
                            [u_max, v_min],
                        ),
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [0., 0., 1.0f32],
                            [u_max, v_max],
                        ),
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [0., 0., 1.0f32],
                            [u_min, v_max],
                        ),
                        // bottom (0., 0., -size)
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [0., 0., -1.0f32],
                            [u_max, v_min],
                        ),
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [0., 0., -1.0f32],
                            [u_min, v_min],
                        ),
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [0., 0., -1.0f32],
                            [u_min, v_max],
                        ),
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [0., 0., -1.0f32],
                            [u_max, v_max],
                        ),
                        // right (size, 0., 0.)
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [1.0f32, 0., 0.],
                            [u_min, v_min],
                        ),
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [1.0f32, 0., 0.],
                            [u_max, v_min],
                        ),
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [1.0f32, 0., 0.],
                            [u_max, v_max],
                        ),
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [1.0f32, 0., 0.],
                            [u_min, v_max],
                        ),
                        // left (-size, 0., 0.)
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [-1.0f32, 0., 0.],
                            [u_max, v_min],
                        ),
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [-1.0f32, 0., 0.],
                            [u_min, v_min],
                        ),
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [-1.0f32, 0., 0.],
                            [u_min, v_max],
                        ),
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [-1.0f32, 0., 0.],
                            [u_max, v_max],
                        ),
                        // front (0., size, 0.)
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [0., 1.0f32, 0.],
                            [u_max, v_min],
                        ),
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [0., 1.0f32, 0.],
                            [u_min, v_min],
                        ),
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [0., 1.0f32, 0.],
                            [u_min, v_max],
                        ),
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y + HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [0., 1.0f32, 0.],
                            [u_max, v_max],
                        ),
                        // back (0., -size, 0.)
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [0., -1.0f32, 0.],
                            [u_min, v_min],
                        ),
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z + HALF_VOXEL_SIZE,
                            ],
                            [0., -1.0f32, 0.],
                            [u_max, v_min],
                        ),
                        (
                            [
                                x - HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [0., -1.0f32, 0.],
                            [u_max, v_max],
                        ),
                        (
                            [
                                x + HALF_VOXEL_SIZE,
                                y - HALF_VOXEL_SIZE,
                                z - HALF_VOXEL_SIZE,
                            ],
                            [0., -1.0f32, 0.],
                            [u_min, v_max],
                        ),
                    ];

                    for (position, normal, uv) in v.iter() {
                        vertices.push(*position);
                        normals.push(*normal);
                        uvs.push(*uv);
                    }

                    let i = vertices.len() - v.len();
                    let offset = i as u32;
                    let local_indices = [
                        offset + 0,
                        offset + 1,
                        offset + 2,
                        offset + 2,
                        offset + 3,
                        offset + 0, // top
                        offset + 4,
                        offset + 5,
                        offset + 6,
                        offset + 6,
                        offset + 7,
                        offset + 4, // bottom
                        offset + 8,
                        offset + 9,
                        offset + 10,
                        offset + 10,
                        offset + 11,
                        offset + 8, // right
                        offset + 12,
                        offset + 13,
                        offset + 14,
                        offset + 14,
                        offset + 15,
                        offset + 12, // left
                        offset + 16,
                        offset + 17,
                        offset + 18,
                        offset + 18,
                        offset + 19,
                        offset + 16, // front
                        offset + 20,
                        offset + 21,
                        offset + 22,
                        offset + 22,
                        offset + 23,
                        offset + 20, // back
                    ];
                    indices.extend(local_indices.iter());
                }
            }
        }
        dbg!(draw_c);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_POSITION), vertices.into());
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_NORMAL), normals.into());
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_UV_0), uvs.into());
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
