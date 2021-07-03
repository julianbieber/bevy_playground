use bevy::prelude::shape::Cube;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::{shader::ShaderDefs, texture::Texture};
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
use cgmath::num_traits::Float;
use itertools::iproduct;
use rand::prelude::*;

#[derive(RenderResources, Default, TypeUuid, ShaderDefs)]
#[uuid = "69e20afc-b1c7-11eb-8529-0242ac130003"]
pub(crate) struct CloudMaterial {
    #[shader_def]
    pub color: Option<Handle<Texture>>,
}

const VERTEX_SHADER: &str = include_str!("cloud.vert");

const FRAGMENT_SHADER: &str = include_str!("cloud.frag");

const TEXTURE_SIZE: usize = 128;
fn create_dots() -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    let mut dots: Vec<Vec3> = Vec::new();
    for _ in (0..10).into_iter() {
        let start = (TEXTURE_SIZE as f32 * 0.1) as i32;
        let end = (TEXTURE_SIZE - (TEXTURE_SIZE as f32 * 0.1) as usize) as i32;
        let x: i32 = rng.gen_range(start..end);
        let y: i32 = rng.gen_range(start..end);
        let z: i32 = rng.gen_range(start..end);
        dots.push(Vec3::new(x as f32, y as f32, z as f32));
    }

    dots
}

fn minimal_distance_to_dots(dot: Vec3, dots: &Vec<Vec3>) -> f32 {
    let mut min = f32::infinity();
    for d in dots.iter() {
        let distance = d.distance(dot) / ((3.0f32 * (TEXTURE_SIZE as f32)).sqrt());
        if distance < min {
            min = distance;
        }
    }
    min
}

fn create_cloud_texture() -> Texture {
    let mut rgba_data = Vec::with_capacity(
        //"F": Floating-point. Thus, GL_RGBA32F is a floating-point format where each component is a 32-bit IEEE floating-point value.
        TEXTURE_SIZE * TEXTURE_SIZE * TEXTURE_SIZE * TextureFormat::Rgba32Float.pixel_size(),
    );
    let mut c = 0;
    let dots = create_dots();
    let dots_r = create_dots();
    let dots_g = create_dots();
    let dots_b = create_dots();
    //calculate minimal distance between actual point and generated dots
    for (x, y, z) in iproduct!(
        (0..TEXTURE_SIZE).into_iter(),
        (0..TEXTURE_SIZE).into_iter(),
        (0..TEXTURE_SIZE).into_iter()
    ) {
        let d = Vec3::new(x as f32, y as f32, z as f32);
        //numbers between 0 and 255
        let r = minimal_distance_to_dots(d, &dots_r) * 255.0;
        let g = minimal_distance_to_dots(d, &dots_g) * 255.0;
        let b = minimal_distance_to_dots(d, &dots_b) * 255.0;
        let alpha = minimal_distance_to_dots(d, &dots) * 255.0;
        //make a bit represenation of integers between 0 and 255
        rgba_data.extend_from_slice(&((r) as u8).to_ne_bytes());
        rgba_data.extend_from_slice(&((g) as u8).to_ne_bytes());
        rgba_data.extend_from_slice(&((b) as u8).to_ne_bytes());
        rgba_data.extend_from_slice(&((alpha) as u8).to_ne_bytes());
        c += 1;
        if c % (TEXTURE_SIZE * TEXTURE_SIZE * TEXTURE_SIZE / 100) == 0 {
            dbg!(c);
        }
    }
    Texture::new(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: TEXTURE_SIZE as u32,
        },
        TextureDimension::D3,
        rgba_data,
        // Red, green, blue, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
        TextureFormat::Rgba8UnormSrgb,
    )
}

pub(crate) fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CloudMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    render_graph.add_system_node(
        "cloud_material",
        AssetRenderResourcesNode::<CloudMaterial>::new(true),
    );

    render_graph
        .add_node_edge("cloud_material", base::node::MAIN_PASS)
        .unwrap();

    let material = materials.add(CloudMaterial {
        color: Option::Some(textures.add(create_cloud_texture())),
    });

    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(from(shape::Cube { size: 30.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            ..Default::default()
        })
        .insert(material);
}

fn from(sp: Cube) -> bevy::prelude::Mesh {
    let vertices = &[
        // Top
        ([-sp.size, -sp.size, sp.size], [0., 0., 1.0], [0., 0., 1.]),
        ([sp.size, -sp.size, sp.size], [0., 0., 1.0], [1.0, 0., 1.0]),
        ([sp.size, sp.size, sp.size], [0., 0., 1.0], [1.0, 1., 1.0]),
        ([-sp.size, sp.size, sp.size], [0., 0., 1.0], [0.0, 1., 1.0]),
        // Bottom
        (
            [-sp.size, sp.size, -sp.size],
            [0., 0., -1.0],
            [0.0, 1., 0.0],
        ),
        ([sp.size, sp.size, -sp.size], [0., 0., -1.0], [1.0, 1., 0.0]),
        (
            [sp.size, -sp.size, -sp.size],
            [0., 0., -1.0],
            [1.0, 0., 0.0],
        ),
        (
            [-sp.size, -sp.size, -sp.size],
            [0., 0., -1.0],
            [0.0, 0., 0.0],
        ),
        // Right
        ([sp.size, -sp.size, -sp.size], [1.0, 0., 0.], [1.0, 0., 0.0]),
        ([sp.size, sp.size, -sp.size], [1.0, 0., 0.], [1.0, 1., 0.0]),
        ([sp.size, sp.size, sp.size], [1.0, 0., 0.], [1.0, 1., 1.0]),
        ([sp.size, -sp.size, sp.size], [1.0, 0., 0.], [1.0, 0., 1.0]),
        // Left
        (
            [-sp.size, -sp.size, sp.size],
            [-1.0, 0., 0.],
            [0.0, 0., 1.0],
        ),
        ([-sp.size, sp.size, sp.size], [-1.0, 0., 0.], [0.0, 1., 1.0]),
        (
            [-sp.size, sp.size, -sp.size],
            [-1.0, 0., 0.],
            [0.0, 1., 0.0],
        ),
        (
            [-sp.size, -sp.size, -sp.size],
            [-1.0, 0., 0.],
            [0.0, 0., 0.0],
        ),
        // Front
        ([sp.size, sp.size, -sp.size], [0., 1.0, 0.], [1.0, 1., 0.0]),
        ([-sp.size, sp.size, -sp.size], [0., 1.0, 0.], [0.0, 1., 0.0]),
        ([-sp.size, sp.size, sp.size], [0., 1.0, 0.], [0.0, 1., 1.0]),
        ([sp.size, sp.size, sp.size], [0., 1.0, 0.], [1.0, 1., 1.0]),
        // Back
        ([sp.size, -sp.size, sp.size], [0., -1.0, 0.], [1.0, 0., 1.0]),
        (
            [-sp.size, -sp.size, sp.size],
            [0., -1.0, 0.],
            [0.0, 0., 1.0],
        ),
        (
            [-sp.size, -sp.size, -sp.size],
            [0., -1.0, 0.],
            [0.0, 0., 0.0],
        ),
        (
            [sp.size, -sp.size, -sp.size],
            [0., -1.0, 0.],
            [1.0, 0., 0.0],
        ),
    ];

    let mut positions = Vec::with_capacity(24);
    let mut normals = Vec::with_capacity(24);
    let mut uvs = Vec::with_capacity(24);

    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let indices = Indices::U32(vec![
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));
    mesh
}
