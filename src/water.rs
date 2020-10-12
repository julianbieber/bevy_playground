use bevy::{
    prelude::*,
    render::{
        mesh::shape,
        pipeline::{DynamicBinding, PipelineDescriptor, PipelineSpecialization, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};
#[derive(RenderResources)]
pub struct WaterMaterial {
    pub time: f32,
}

impl WaterMaterial {
    pub fn add(&mut self) {
        self.time += 0.1f32;
    }
}

pub fn update_material_time(
    mut material: ResMut<Assets<WaterMaterial>>,
    handle: Mut<Handle<WaterMaterial>>,
) {
    for m in material.get_mut(&handle).iter_mut() {
        m.add();
    }
}

pub struct WaterEffected;

pub fn setup_water_layer(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    asset_server: Res<AssetServer>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
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

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("water_material", base::node::MAIN_PASS)
        .unwrap();

    // Create a new material
    let material = materials.add(WaterMaterial { time: 0.0f32 });
    let mesh = asset_server.load("assets/water.gltf").unwrap();

    // Setup our world
    commands
        // cube
        .spawn(MeshComponents {
            mesh: mesh,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                pipeline_handle,
                // NOTE: in the future you wont need to manually declare dynamic bindings
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 1,
                            binding: 0,
                        },
                        // MyMaterial_color
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
        .with(material);
}

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 1, binding = 1) uniform WaterMaterial_time {
    float time;
};
void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position.x, Vertex_Position.y + sin(time * 0.1 + Vertex_Position.x) + sin(time * 0.1 + Vertex_Position.z + 0.5), Vertex_Position.z, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
layout(set = 1, binding = 1) uniform WaterMaterial_time {
    float time;
};
void main() {
    o_Target = vec4(0, 0, (sin(gl_FragCoord.x + gl_FragCoord.y + time) + 1) / 2 * 255, 1.0);
}
"#;
