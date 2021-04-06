use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use std::borrow::Cow;

use super::{chunk::VoxelChunk, lod::combine_voxels, voxel::VoxelTypes};

impl From<&VoxelChunk> for Mesh {
    fn from(chunk: &VoxelChunk) -> Self {
        let faces = combine_voxels(chunk);

        let vertices_count = (faces.len()) * 4;
        let mut indices: Vec<u32> = Vec::with_capacity(vertices_count * 6);
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);

        let mut current_index = 0;

        for face in faces {
            match face.direction {
                super::voxel::VoxelDirection::UP => {
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y + face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y + face.size,
                        face.center.z + face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y + face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y + face.size,
                        face.center.z + face.size,
                    ]);
                    normals.push([0.0, 1.0, 0.0]);
                    normals.push([0.0, 1.0, 0.0]);
                    normals.push([0.0, 1.0, 0.0]);
                    normals.push([0.0, 1.0, 0.0]);
                    indices.push(current_index + 0);
                    indices.push(current_index + 1);
                    indices.push(current_index + 2);

                    indices.push(current_index + 1);
                    indices.push(current_index + 3);
                    indices.push(current_index + 2);
                }
                super::voxel::VoxelDirection::DOWN => {
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y - face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y - face.size,
                        face.center.z + face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y - face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y - face.size,
                        face.center.z + face.size,
                    ]);
                    normals.push([0.0, -1.0, 0.0]);
                    normals.push([0.0, -1.0, 0.0]);
                    normals.push([0.0, -1.0, 0.0]);
                    normals.push([0.0, -1.0, 0.0]);

                    indices.push(current_index + 0);
                    indices.push(current_index + 2);
                    indices.push(current_index + 1);

                    indices.push(current_index + 3);
                    indices.push(current_index + 1);
                    indices.push(current_index + 2);
                }
                super::voxel::VoxelDirection::LEFT => {
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y - face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y + face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y - face.size,
                        face.center.z + face.size,
                    ]);
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y + face.size,
                        face.center.z + face.size,
                    ]);
                    normals.push([-1.0, 0.0, 0.0]);
                    normals.push([-1.0, 0.0, 0.0]);
                    normals.push([-1.0, 0.0, 0.0]);
                    normals.push([-1.0, 0.0, 0.0]);
                    indices.push(current_index + 0);
                    indices.push(current_index + 2);
                    indices.push(current_index + 1);

                    indices.push(current_index + 3);
                    indices.push(current_index + 1);
                    indices.push(current_index + 2);
                }
                super::voxel::VoxelDirection::RIGHT => {
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y - face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y + face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y - face.size,
                        face.center.z + face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y + face.size,
                        face.center.z + face.size,
                    ]);

                    normals.push([1.0, 0.0, 0.0]);
                    normals.push([1.0, 0.0, 0.0]);
                    normals.push([1.0, 0.0, 0.0]);
                    normals.push([1.0, 0.0, 0.0]);

                    indices.push(current_index + 0);
                    indices.push(current_index + 1);
                    indices.push(current_index + 2);

                    indices.push(current_index + 1);
                    indices.push(current_index + 3);
                    indices.push(current_index + 2);
                }
                super::voxel::VoxelDirection::FRONT => {
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y - face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y + face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y - face.size,
                        face.center.z - face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y + face.size,
                        face.center.z - face.size,
                    ]);

                    normals.push([0.0, 0.0, -1.0]);
                    normals.push([0.0, 0.0, -1.0]);
                    normals.push([0.0, 0.0, -1.0]);
                    normals.push([0.0, 0.0, -1.0]);

                    indices.push(current_index + 0);
                    indices.push(current_index + 1);
                    indices.push(current_index + 2);

                    indices.push(current_index + 1);
                    indices.push(current_index + 3);
                    indices.push(current_index + 2);
                }
                super::voxel::VoxelDirection::BACK => {
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y - face.size,
                        face.center.z + face.size,
                    ]);
                    vertices.push([
                        face.center.x - face.size,
                        face.center.y + face.size,
                        face.center.z + face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y - face.size,
                        face.center.z + face.size,
                    ]);
                    vertices.push([
                        face.center.x + face.size,
                        face.center.y + face.size,
                        face.center.z + face.size,
                    ]);

                    normals.push([0.0, 0.0, 1.0]);
                    normals.push([0.0, 0.0, 1.0]);
                    normals.push([0.0, 0.0, 1.0]);
                    normals.push([0.0, 0.0, 1.0]);

                    indices.push(current_index + 0);
                    indices.push(current_index + 2);
                    indices.push(current_index + 1);

                    indices.push(current_index + 3);
                    indices.push(current_index + 1);
                    indices.push(current_index + 2);
                }
            }
            let (u_min, u_max, v_min, v_max) = uvs_from_typ(&face.typ);
            uvs.push([u_min, v_min]);
            uvs.push([u_min, v_max]);
            uvs.push([u_max, v_min]);
            uvs.push([u_max, v_max]);
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
