use ahash::{AHashMap, AHashSet};
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
};

use crate::voxel_world::voxel::{Voxel, VoxelPosition, HALF_VOXEL_SIZE};

use super::water_shaders::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use std::{borrow::Cow, collections::VecDeque};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
pub struct WaterMaterial {
    pub time: f32,
}

impl WaterMaterial {
    pub fn add(&mut self, time: f32) {
        let diff = std::f32::consts::PI * 20.0 - self.time;
        if diff < 0.0 {
            self.time -= std::f32::consts::PI * 20.0;
        }
        self.time += time * 5.0;
    }
}

pub fn update_material_time(mut material: ResMut<Assets<WaterMaterial>>, time: Res<Time>) {
    let handles: Vec<_> = material.ids().collect();
    for handle in handles.into_iter() {
        material.get_mut(handle).unwrap().add(time.delta_seconds());
    }
}

pub fn setup_water_object(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    render_graph.add_system_node(
        "water_material",
        AssetRenderResourcesNode::<WaterMaterial>::new(true),
    );

    render_graph
        .add_node_edge("water_material", base::node::MAIN_PASS)
        .unwrap();

    let mut water = Water::new();
    let mesh = meshes.add(water.initial_mesh());

    water.add(VoxelPosition::new(0, 20, 0));

    let material = materials.add(WaterMaterial { time: 0.0f32 });
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));

    commands
        .spawn_bundle(MeshBundle {
            mesh,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform,
            visible: Visible {
                is_transparent: true,
                is_visible: true,
            },
            ..Default::default()
        })
        .insert(water)
        .insert(material);
}

pub fn update_water_mesh(
    mut water_query: Query<(&mut Water, &Handle<Mesh>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut water, handle_current_mesh) in water_query.iter_mut() {
        if let Some(mut current_mesh) = meshes.get_mut(handle_current_mesh) {
            water.update_mesh(&mut current_mesh);
        }
    }
}

pub struct Water {
    voxels: AHashMap<VoxelPosition, WaterVoxel>,
    added: AHashSet<VoxelPosition>,
    removed: AHashSet<VoxelPosition>,
    unused: VecDeque<[u32; 4]>,
}

/*
Fixed index buffer builds quads from all vertices.
When adding/removing values, only Vertex positions and normals have to be updated.
 */
struct WaterVoxel {
    indices: Vec<[u32; 4]>,
}

const UNUSED: f32 = 100000000.0;
const WATER_QUADS: usize = 1024;
const VERTEX_BUFFER_SIZE: usize = 4 * WATER_QUADS;

impl Water {
    fn new() -> Water {
        let mut unused = VecDeque::with_capacity(WATER_QUADS);
        for i in 0..WATER_QUADS {
            unused.push_back([
                i as u32 * 4 + 0,
                i as u32 * 4 + 1,
                i as u32 * 4 + 2,
                i as u32 * 4 + 3,
            ]);
        }
        Water {
            voxels: AHashMap::new(),
            added: AHashSet::new(),
            removed: AHashSet::new(),
            unused,
        }
    }

    fn add(&mut self, p: VoxelPosition) {
        self.added.insert(p);
    }

    fn remove(&mut self, p: VoxelPosition) {
        self.removed.insert(p);
    }

    fn initial_mesh(&self) -> Mesh {
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

    fn update_mesh(&mut self, mut mesh: &mut Mesh) {
        let mut actually_added = AHashSet::new();
        for a in self.added.iter() {
            if !self.removed.remove(a) {
                actually_added.insert(a.clone());
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

            for removed in self.removed.iter() {
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
                // TOP
                let indices = dbg!(self.unused.pop_back().unwrap());
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
                dbg!(i);
                normals[i as usize] = [0.0, 1.0, 0.0];
            }
            for i in bottom_indices {
                dbg!(i);
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

        self.removed = AHashSet::new();
        self.added = AHashSet::new();
    }
}
