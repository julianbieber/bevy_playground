use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use std::borrow::Cow;

use super::{
    chunk::{self, VoxelChunk},
    voxel::{Voxel, VoxelTypes, HALF_VOXEL_SIZE},
};

impl From<&VoxelChunk> for Mesh {
    fn from(chunk: &VoxelChunk) -> Self {
        let mut top = Vec::new();
        let mut bottom = Vec::new();
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut front = Vec::new();
        let mut back = Vec::new();

        for voxel in chunk.get_voxels().iter() {
            let surrounding = voxel.position.surrounding();

            if chunk.get(&surrounding.top).is_none() {
                top.push(voxel.clone());
            }
            if chunk.get(&surrounding.bottom).is_none() {
                bottom.push(voxel.clone());
            }
            if chunk.get(&surrounding.left).is_none() {
                left.push(voxel.clone());
            }
            if chunk.get(&surrounding.right).is_none() {
                right.push(voxel.clone());
            }
            if chunk.get(&surrounding.front).is_none() {
                front.push(voxel.clone());
            }
            if chunk.get(&surrounding.back).is_none() {
                back.push(voxel.clone());
            }
        }
        let vertices_count =
            (top.len() + bottom.len() + left.len() + right.len() + front.len() + back.len()) * 4;
        let mut indices: Vec<u32> = Vec::with_capacity(vertices_count * 6);
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);

        let mut current_index = 0;

        for voxel in top.iter() {
            let base = voxel.position.to_vec();
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);

            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);
            normals.push([0.0, 1.0, 0.0]);

            let (u_min, u_max, v_min, v_max) = uvs_from_typ(&voxel.typ);

            uvs.push([u_min, v_min]);
            uvs.push([u_min, v_max]);
            uvs.push([u_max, v_min]);
            uvs.push([u_max, v_max]);

            indices.push(current_index + 0);
            indices.push(current_index + 1);
            indices.push(current_index + 2);

            indices.push(current_index + 1);
            indices.push(current_index + 3);
            indices.push(current_index + 2);

            current_index += 4;
        }

        for voxel in bottom.iter() {
            let base = voxel.position.to_vec();
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);

            normals.push([0.0, -1.0, 0.0]);
            normals.push([0.0, -1.0, 0.0]);
            normals.push([0.0, -1.0, 0.0]);
            normals.push([0.0, -1.0, 0.0]);

            let (u_min, u_max, v_min, v_max) = uvs_from_typ(&voxel.typ);

            uvs.push([u_min, v_min]);
            uvs.push([u_min, v_max]);
            uvs.push([u_max, v_min]);
            uvs.push([u_max, v_max]);

            indices.push(current_index + 0);
            indices.push(current_index + 2);
            indices.push(current_index + 1);

            indices.push(current_index + 3);
            indices.push(current_index + 1);
            indices.push(current_index + 2);

            current_index += 4;
        }

        for voxel in left.iter() {
            let base = voxel.position.to_vec();
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);

            normals.push([-1.0, 0.0, 0.0]);
            normals.push([-1.0, 0.0, 0.0]);
            normals.push([-1.0, 0.0, 0.0]);
            normals.push([-1.0, 0.0, 0.0]);

            let (u_min, u_max, v_min, v_max) = uvs_from_typ(&voxel.typ);

            uvs.push([u_min, v_min]);
            uvs.push([u_min, v_max]);
            uvs.push([u_max, v_min]);
            uvs.push([u_max, v_max]);

            indices.push(current_index + 0);
            indices.push(current_index + 2);
            indices.push(current_index + 1);

            indices.push(current_index + 3);
            indices.push(current_index + 1);
            indices.push(current_index + 2);

            current_index += 4;
        }

        for voxel in right.iter() {
            let base = voxel.position.to_vec();
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);

            normals.push([1.0, 0.0, 0.0]);
            normals.push([1.0, 0.0, 0.0]);
            normals.push([1.0, 0.0, 0.0]);
            normals.push([1.0, 0.0, 0.0]);

            let (u_min, u_max, v_min, v_max) = uvs_from_typ(&voxel.typ);

            uvs.push([u_min, v_min]);
            uvs.push([u_min, v_max]);
            uvs.push([u_max, v_min]);
            uvs.push([u_max, v_max]);

            indices.push(current_index + 0);
            indices.push(current_index + 1);
            indices.push(current_index + 2);

            indices.push(current_index + 1);
            indices.push(current_index + 3);
            indices.push(current_index + 2);

            current_index += 4;
        }

        for voxel in front.iter() {
            let base = voxel.position.to_vec();
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z - HALF_VOXEL_SIZE,
            ]);

            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);

            let (u_min, u_max, v_min, v_max) = uvs_from_typ(&voxel.typ);

            uvs.push([u_min, v_min]);
            uvs.push([u_min, v_max]);
            uvs.push([u_max, v_min]);
            uvs.push([u_max, v_max]);

            indices.push(current_index + 0);
            indices.push(current_index + 1);
            indices.push(current_index + 2);

            indices.push(current_index + 1);
            indices.push(current_index + 3);
            indices.push(current_index + 2);

            current_index += 4;
        }

        for voxel in back.iter() {
            let base = voxel.position.to_vec();
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x - HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y - HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);
            vertices.push([
                base.x + HALF_VOXEL_SIZE,
                base.y + HALF_VOXEL_SIZE,
                base.z + HALF_VOXEL_SIZE,
            ]);

            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);
            normals.push([0.0, 0.0, -1.0]);

            let (u_min, u_max, v_min, v_max) = uvs_from_typ(&voxel.typ);

            uvs.push([u_min, v_min]);
            uvs.push([u_min, v_max]);
            uvs.push([u_max, v_min]);
            uvs.push([u_max, v_max]);

            indices.push(current_index + 0);
            indices.push(current_index + 2);
            indices.push(current_index + 1);

            indices.push(current_index + 3);
            indices.push(current_index + 1);
            indices.push(current_index + 2);

            current_index += 4;
        }

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
        VoxelTypes::DarkRock1 => (0.0f32, 0.25f32, 0.5f32, 1.0f32),
        VoxelTypes::DarkRock2 => (0.25f32, 0.5f32, 0.5f32, 1.0f32),
        VoxelTypes::Lava => (0.25, 0.5, 0.0, 0.5),
        VoxelTypes::Moss => (0.5, 0.75, 0.0, 0.5),
        VoxelTypes::LightRock1 => (0.75, 1.0, 0.0, 0.5),
        VoxelTypes::LightRock2 => (0.5, 0.75, 0.5, 1.0),
        VoxelTypes::CrackedRock => (0.0, 0.25, 0.0, 0.5),
    }
}
