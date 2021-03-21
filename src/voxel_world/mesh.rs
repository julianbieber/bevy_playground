use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use std::borrow::Cow;

use super::voxel::{Voxel, VoxelTypes, HALF_VOXEL_SIZE};

impl From<&Voxel> for Mesh {
    fn from(voxel: &Voxel) -> Self {
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let (u_min, u_max, v_min, v_max) = uvs_from_typ(&voxel.typ);

        let v = voxel_vertices(0.0, 0.0, 0.0, u_min, u_max, v_min, v_max);

        for (position, normal, uv) in v.iter() {
            vertices.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let local_indices = [
            0, 1, 2, 2, 3, 0, // top
            4, 5, 6, 6, 7, 4, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ];
        indices.extend(local_indices.iter());

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_POSITION), vertices);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_NORMAL), normals);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_UV_0), uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}

fn uvs_from_typ(typ: &VoxelTypes) -> (f32, f32, f32, f32) {
    match typ {
        VoxelTypes::DarkRock1 => (0.0, 0.125, 0.0, 1.0),
        VoxelTypes::Moss => (0.125, 0.25, 0.0, 1.0),
        VoxelTypes::GreyRock1 => (0.25, 0.375, 0.0, 1.0),
        VoxelTypes::GreyRock2 => (0.375, 0.5, 0.0, 1.0),
        VoxelTypes::BrownRock => (0.5, 0.625, 0.0, 1.0),
        VoxelTypes::DarkRock2 => (0.625, 0.75, 0.0, 1.0),
        VoxelTypes::GroundRock1 => (0.75, 0.875, 0.0, 1.0),
        VoxelTypes::Snow => (0.875, 1.0, 0.0, 1.0),
    }
}

fn voxel_vertices(
    x: f32,
    y: f32,
    z: f32,
    u_min: f32,
    u_max: f32,
    v_min: f32,
    v_max: f32,
) -> Vec<([f32; 3], [f32; 3], [f32; 2])> {
    vec![
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
    ]
}
