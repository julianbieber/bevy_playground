use bevy::prelude::shape::Icosphere;
use bevy::render::pipeline::FrontFace;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};
use common::PlayerPosition;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "69e20afc-b1c7-11eb-8529-0242ac130003"]

// Kept
pub struct IrrelevantMaterial {
    pub irrelevant: f32,
}

const VERTEX_SHADER: &str = include_str!("cloud.vert");

const FRAGMENT_SHADER: &str = include_str!("cloud.frag");

pub(crate) fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<IrrelevantMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let mut pipeline_descriptor = PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    });

    // render only inside of Icosphere (See: https://www.khronos.org/opengl/wiki/Face_Culling)
    pipeline_descriptor.primitive.front_face = FrontFace::Cw;
    let pipeline_handle = pipelines.add(pipeline_descriptor);

    render_graph.add_system_node(
        "irrelevant_material",
        AssetRenderResourcesNode::<IrrelevantMaterial>::new(true),
    );
    render_graph
        .add_node_edge("irrelevant_material", base::node::MAIN_PASS)
        .unwrap();

    let material = materials.add(IrrelevantMaterial { irrelevant: 0.0 });
    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(From::<Icosphere>::from(shape::Icosphere {
                radius: 55.0,
                subdivisions: 10,
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 22.0, 0.0),
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            ..Default::default()
        })
        .insert(material)
        .insert(Cloud {});
}

pub fn update_cloud_positon(
    mut cloud_query: Query<(Entity, &Cloud, &mut Transform)>,
    player_postion: Res<PlayerPosition>,
) {
    for (_, _, mut cloud_transform) in cloud_query.iter_mut() {
        cloud_transform.translation.x = player_postion.position.x;
        cloud_transform.translation.z = player_postion.position.z;
    }
}
pub struct Cloud {}
