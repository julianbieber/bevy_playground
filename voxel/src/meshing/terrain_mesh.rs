use std::borrow::Cow;

use bevy::{
    prelude::Mesh,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};

use crate::{voxel::VoxelDirection,voxel::{Voxel}, world_sector::WorldSector};

use super::{
    consts::VERTICES_PER_MESH,
    util::{to_normals, to_tangents, to_vertices, uvs_from_typ},
};

pub trait Meshing<const CHUNKS_LOADED: i32, const CHUNK_SIZE: i32> {
    fn initial_terrain_meshes(&self) -> Vec<Mesh>;
}

impl<const CHUNKS_LOADED: i32, const CHUNK_SIZE: i32> Meshing<CHUNKS_LOADED, CHUNK_SIZE>
    for WorldSector<CHUNKS_LOADED, CHUNK_SIZE>
{
    fn initial_terrain_meshes(&self) -> Vec<Mesh> {
        let directions = [
            VoxelDirection::UP,
            VoxelDirection::DOWN,
            VoxelDirection::LEFT,
            VoxelDirection::RIGHT,
            VoxelDirection::BACK,
            VoxelDirection::FRONT,
        ];

        let mut meshes = Vec::new();
        let mut indices: Vec<u32> = Vec::with_capacity(VERTICES_PER_MESH * 6);
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(VERTICES_PER_MESH);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(VERTICES_PER_MESH);
        let mut tangents: Vec<[f32; 4]> = Vec::with_capacity(VERTICES_PER_MESH);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(VERTICES_PER_MESH);
        let mut index = 0;

        for (chunk_i, chunk) in self.chunks.iter().enumerate() {
            for (voxel_i, voxel) in chunk.voxels.iter().enumerate() {
                match voxel {
                    Voxel::LandVoxel { typ } => {
                        let surrounding = self.get_surrounding(chunk_i, voxel_i, directions);
                        for (d, v_o) in surrounding {
                            let render_face = if let Some(v) = v_o {
                                match v {
                                    crate::voxel::Voxel::LandVoxel { .. } => false,
                                    crate::voxel::Voxel::WaterVoxel { .. } => true,
                                    crate::voxel::Voxel::Nothing => true,
                                }
                            } else {
                                false
                            };

                            if render_face {
                                let position =
                                    self.chunks[chunk_i].index_to_coord(voxel_i).to_vec();
                                vertices.extend_from_slice(&to_vertices(d, position));
                                normals.extend_from_slice(&to_normals(d));
                                tangents.extend_from_slice(&to_tangents(d));
                                uvs.extend_from_slice(&uvs_from_typ(typ));

                                indices.extend_from_slice(&[
                                    index * 4 + 0,
                                    index * 4 + 1,
                                    index * 4 + 2,
                                    index * 4 + 0,
                                    index * 4 + 2,
                                    index * 4 + 3,
                                ]);
                                index += 1;

                                if vertices.len() >= VERTICES_PER_MESH {
                                    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                                    mesh.set_attribute(
                                        Cow::Borrowed(Mesh::ATTRIBUTE_POSITION),
                                        vertices,
                                    );
                                    mesh.set_attribute(
                                        Cow::Borrowed(Mesh::ATTRIBUTE_NORMAL),
                                        normals,
                                    );
                                    mesh.set_attribute(
                                        Cow::Borrowed(Mesh::ATTRIBUTE_TANGENT),
                                        tangents,
                                    );
                                    mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_UV_0), uvs);
                                    mesh.set_indices(Some(Indices::U32(indices)));
                                    meshes.push(mesh);

                                    indices = Vec::with_capacity(VERTICES_PER_MESH * 6);
                                    vertices = Vec::with_capacity(VERTICES_PER_MESH);
                                    normals = Vec::with_capacity(VERTICES_PER_MESH);
                                    tangents = Vec::with_capacity(VERTICES_PER_MESH);
                                    uvs = Vec::with_capacity(VERTICES_PER_MESH);
                                    index = 0;
                                }
                            }
                        }
                    }
                    Voxel::WaterVoxel { .. } => (),
                    Voxel::Nothing => (),
                };
            }
        }

        vertices.shrink_to_fit();
        normals.shrink_to_fit();
        tangents.shrink_to_fit();
        uvs.shrink_to_fit();
        indices.shrink_to_fit();
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_POSITION), vertices);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_NORMAL), normals);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_TANGENT), tangents);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_UV_0), uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        meshes.push(mesh);

        dbg!(meshes.len());

        meshes
    }
}
