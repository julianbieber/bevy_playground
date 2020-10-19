use bevy::{
    prelude::*,
    render::{
        pipeline::{DynamicBinding, PipelineDescriptor, PipelineSpecialization, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

use crate::water::water_shaders::*;

#[derive(RenderResources)]
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
    size: Vec2
}

impl WaterPosition {
    /// returns (min_x, min_z) (max_x, max_z)
    fn get_boundaries(&self) -> (Vec2, Vec2) {
        let x_half_size = self.size.x() / 2.0;
        let z_half_size = self.size.y() / 2.0;
        (
            Vec2::new(self.position.x() - x_half_size, self.position.z() - z_half_size),
            Vec2::new(self.position.x() + x_half_size, self.position.z() + z_half_size)
        )
    }

    pub fn lies_within(&self, point: Vec3) -> bool {
        let (min, max) = self.get_boundaries();
        point.x() >= min.x() && point.z() >= min.y() && point.x() <= max.x() && point.z() <= max.y()
    }
}

pub fn update_material_time(
    mut material: ResMut<Assets<WaterMaterial>>,
    time: Res<Time>,
    handle: Mut<Handle<WaterMaterial>>,
) {
    for m in material.get_mut(&handle).iter_mut() {
        m.add(time.delta_seconds);
    }
}

pub fn set_water_position(mut position: Mut<WaterPosition>, transform: &Transform) {
    position.position = transform.translation();
    position.size = Vec2::new(transform.scale().x(), transform.scale().z());
}

pub fn setup_water_layer(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    asset_server: Res<AssetServer>,
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

    let material = materials.add(WaterMaterial { time: 0.0f32 });
    let mesh = asset_server.load("assets/water.gltf").unwrap();

    commands
        .spawn(MeshComponents {
            mesh: mesh,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                pipeline_handle,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        DynamicBinding {
                            bind_group: 1,
                            binding: 0,
                        },
                        DynamicBinding {
                            bind_group: 1,
                            binding: 1,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_non_uniform_scale(Vec3::new(10.0, 1.0, 10.0)),
            ..Default::default()
        })
        .with(material)
        .with(WaterPosition {
            position: Vec3::zero(),
            size: Vec2::zero()
        });
}
