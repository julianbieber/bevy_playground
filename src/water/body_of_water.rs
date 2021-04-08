use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

use crate::water::water_shaders::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use std::borrow::Cow;

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

pub struct WaterPosition {
    position: Vec3,
    size: Vec2,
}

impl WaterPosition {
    /// returns (min_x, min_z) (max_x, max_z)
    fn get_boundaries(&self) -> (Vec2, Vec2) {
        let x_half_size = self.size.x / 2.0;
        let z_half_size = self.size.y / 2.0;
        (
            Vec2::new(self.position.x - x_half_size, self.position.z - z_half_size),
            Vec2::new(self.position.x + x_half_size, self.position.z + z_half_size),
        )
    }

    pub fn lies_within(&self, point: Vec3) -> bool {
        let (min, max) = self.get_boundaries();
        point.x >= min.x && point.z >= min.y && point.x <= max.x && point.z <= max.y
    }
}

pub fn update_material_time(mut material: ResMut<Assets<WaterMaterial>>, time: Res<Time>) {
    let handles: Vec<_> = material.ids().collect();
    for handle in handles.into_iter() {
        material.get_mut(handle).unwrap().add(time.delta_seconds());
    }
}

pub fn set_water_position(mut position_2_transform_query: Query<(&mut WaterPosition, &Transform)>) {
    for (mut position, transform) in position_2_transform_query.iter_mut() {
        position.position = transform.translation;
        position.size = Vec2::new(transform.scale.x, transform.scale.z);
    }
}

pub fn setup_water_layer(
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

    let water = Water::new((50.0, 50.0), 1000);
    let mesh = meshes.add(Mesh::from(water));

    let material = materials.add(WaterMaterial { time: 0.0f32 });
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    transform.scale = Vec3::new(10.0, 1.0, 10.0);
    commands
        .spawn_bundle(MeshBundle {
            mesh,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform,
            ..Default::default()
        })
        .insert(material)
        .insert(WaterPosition {
            position: Vec3::ZERO,
            size: Vec2::ZERO,
        });
}

struct Water {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl Water {
    fn new(size: (f32, f32), splits: u32) -> Water {
        let mut vertices = Vec::new();
        let lower_left = (size.0 / -2.0, 0.0, size.1 / -2.0);
        let split_length_x = size.0 / splits as f32;
        let split_length_z = size.1 / splits as f32;
        for x_split in 0..(splits + 1) {
            let x = lower_left.0 + x_split as f32 * split_length_x;
            for z_split in 0..(splits + 1) {
                let z = lower_left.0 + z_split as f32 * split_length_z;
                vertices.push([x, 0.0f32, z]);
            }
        }

        let normals = vertices.iter().map(|_| [0.0f32, 1.0f32, 0.0f32]).collect();

        let uvs = vertices
            .iter()
            .map(|_| {
                [0.0f32, 0.0f32] // TODO actual uv coordinates
            })
            .collect();

        let mut indices = Vec::new();
        for first in 0..(splits + 1) * splits {
            if first % (splits + 1) != splits {
                let second = first + 1;
                let third = first + splits + 1;
                indices.push(first);
                indices.push(second);
                indices.push(third);
            }
        }

        for first in 1..(splits + 1) * splits {
            if first % (splits + 1) != 0 {
                let second = first + splits + 1;
                let third = first + splits;
                indices.push(first);
                indices.push(second);
                indices.push(third);
            }
        }

        Water {
            vertices,
            normals,
            uvs,
            indices,
        }
    }
}

impl From<Water> for Mesh {
    fn from(water: Water) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_POSITION), water.vertices);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_NORMAL), water.normals);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_UV_0), water.uvs);
        mesh.set_indices(Some(Indices::U32(water.indices)));
        mesh
    }
}
