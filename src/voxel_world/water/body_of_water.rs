use std::time::Duration;

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

use crate::voxel_world::{
    access::VoxelAccess,
    chunk::VoxelChunk,
    voxel::{VoxelDirection, VoxelPosition},
    water::water_source::WaterSource,
};

use super::{
    water::{Water, WaterOperations},
    water_shaders::*,
};

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

    let water = Water::new();
    let mesh = meshes.add(water.initial_mesh());

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

    commands.spawn().insert(WaterSource::new());
}

pub fn internal_water_physics(
    water_query: Query<(&Water,)>,
    mut water_operations: ResMut<WaterOperations>,
    voxel_access: Res<VoxelAccess>,
) {
    for (water,) in water_query.iter() {
        for (position, _) in water.voxels.iter() {
            // fall down
            let down = position.in_direction(VoxelDirection::DOWN);
            let potential_new_position = [
                down
            ];

            for new in potential_new_position.iter() {
                if water.voxels.get(&new).is_none()
                    && voxel_access.get_voxel(new.clone()).is_none()
                    && water_operations.add(new.clone())
                {
                    water_operations.remove(position.clone());
                    break;
                }
            }

            let pressure_checks = [
                (VoxelDirection::LEFT, VoxelDirection::RIGHT),
                (VoxelDirection::FRONT, VoxelDirection::BACK),
                (VoxelDirection::RIGHT, VoxelDirection::LEFT),
                (VoxelDirection::BACK, VoxelDirection::FRONT),
            ];

            let under_water = water.voxels.get(&position.in_direction(VoxelDirection::UP)).is_some();
            for (from, to) in pressure_checks.iter() {
                let same_level = position.in_direction(from.clone());
                let destination = position.in_direction(to.clone());
                if (under_water || water.voxels.get(&same_level).is_some())
                    && water.voxels.get(&destination).is_none()
                    && voxel_access.get_voxel(destination.clone()).is_none()
                    && water_operations.add(destination)
                {
                    water_operations.remove(position.clone());
                    break;
                }
            }
        }
    }
}

pub fn update_water_mesh(
    mut water_query: Query<(&mut Water, &Handle<Mesh>)>,
    mut water_operations: ResMut<WaterOperations>,
    mut meshes: ResMut<Assets<Mesh>>,
    voxel_access: Res<VoxelAccess>,
) {
    for (mut water, handle_current_mesh) in water_query.iter_mut() {
        if let Some(mut current_mesh) = meshes.get_mut(handle_current_mesh) {
            water.update_mesh(&mut current_mesh, &mut water_operations, &voxel_access);
        }
    }
}
