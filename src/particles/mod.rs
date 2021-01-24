mod primitives;

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
    tasks::AsyncComputeTaskPool,
};

use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use primitives::*;
use rand::{thread_rng, Rng};

use flume::{unbounded, Receiver, Sender};
use std::time::Duration;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (tx, rx) = unbounded::<(Mesh, Explosion)>();
        app.add_resource(ExplosionSpawnCoolDown {
            timer: Timer::from_seconds(0.1, true),
        })
        .add_asset::<ParticleDirectionMaterial>()
        .add_resource(tx)
        .add_resource(rx)
        .init_resource::<DelayedParticleSpawns>()
        .add_startup_system(setup_particles.system())
        .add_system(spawn_regular_explosions_system.system())
        .add_system(despawn_explosions.system())
        .add_system(update_particle_direction.system())
        .add_system(evaluate_delayed_particles.system())
        .add_system(spawn_from_channel.system());
    }
}

fn setup_particles(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    render_graph.add_system_node(
        "particle",
        AssetRenderResourcesNode::<ParticleDirectionMaterial>::new(true),
    );

    render_graph
        .add_node_edge("particle", base::node::MAIN_PASS)
        .unwrap();
    commands.insert_resource(ParticlePipeline {
        handle: pipeline_handle,
    });
}

struct ParticlePipeline {
    handle: Handle<PipelineDescriptor>,
}

#[derive(Default)]
pub struct DelayedParticleSpawns {
    pub spawns: Vec<(Timer, Explosion)>,
}

#[derive(Clone)]
pub struct Explosion {
    pub duration: Duration,
    pub radius: f32,
    pub particles: u32,
    pub position: Vec3,
}

struct ExplosionSpawnCoolDown {
    pub timer: Timer,
}

fn evaluate_delayed_particles(
    mut delayed_particle_spawns_res: ResMut<DelayedParticleSpawns>,
    time: Res<Time>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<(Mesh, Explosion)>>,
) {
    let mut at_least_one = false;
    for (timer, explosion) in delayed_particle_spawns_res.spawns.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            let e = explosion.clone();
            let tx_cloned = tx.clone();
            pool.spawn(async move {
                let mesh = create_explosion_mesh(&e);
                tx_cloned.send((mesh, e)).unwrap();
            })
            .detach();
            at_least_one = true;
        }
    }

    if at_least_one {
        let remaining: Vec<(Timer, Explosion)> = delayed_particle_spawns_res
            .spawns
            .iter()
            .filter(|(t, _)| !t.just_finished())
            .map(|(t, e)| (t.clone(), e.clone()))
            .collect();

        delayed_particle_spawns_res.spawns = remaining;
    }
}

fn spawn_regular_explosions_system(
    mut spawn_timer: ResMut<ExplosionSpawnCoolDown>,
    time: Res<Time>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<(Mesh, Explosion)>>,
) {
    if spawn_timer.timer.tick(time.delta_seconds()).just_finished() {
        spawn_timer.timer.reset();
        let tx_copy = tx.clone();
        pool.0
            .spawn(async move {
                let e = Explosion {
                    duration: Duration::from_secs(2),
                    radius: 10.0,
                    particles: 10000,
                    position: Vec3::new(
                        thread_rng().gen_range(-100.0f32, 100.0f32),
                        thread_rng().gen_range(0.0f32, 100.0f32),
                        thread_rng().gen_range(-100.0f32, 100.0f32),
                    ),
                };
                let mesh = create_explosion_mesh(&e);
                tx_copy.send((mesh, e)).unwrap();
            })
            .detach();
    }
}

fn despawn_explosions(
    commands: &mut Commands,
    mut explosions_query: Query<(Entity, &ExplosionMarker, &mut Timer)>,
    time: Res<Time>,
) {
    for (e, _, mut timer) in explosions_query.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            commands.despawn(e);
        }
    }
}

fn create_explosion_mesh(explosion: &Explosion) -> Mesh {
    let particle_count = explosion.particles;
    let radius = explosion.radius;
    let cube_vertices = cube_vertices(0.02);
    let mut positions = Vec::with_capacity(24 * particle_count as usize);
    let mut indices = Vec::with_capacity(36 * particle_count as usize);
    let mut particle_directions = Vec::with_capacity(24 * particle_count as usize);
    for i in 0..particle_count {
        positions.extend_from_slice(&cube_vertices);
        indices.extend(cube_indices(i).iter());
    }
    for _ in 0..particle_count {
        let mut x: f32 = thread_rng().gen_range(-radius, radius);
        let mut y: f32 = thread_rng().gen_range(-radius, radius);
        let mut z: f32 = thread_rng().gen_range(-radius, radius);
        let div = (x * x + y * y + z * z).sqrt();
        x /= div;
        y /= div;
        z /= div;
        let d = thread_rng().gen_range(0.0, 1.0);

        for _ in 0..8 {
            particle_directions.push([x * d, y * d, z * d]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh.set_attribute(
        "Particle_Direction",
        VertexAttributeValues::from(particle_directions),
    );
    mesh
}

fn spawn_from_channel(
    commands: &mut Commands,
    particle_pipeline: Res<ParticlePipeline>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleDirectionMaterial>>,
    rx: Res<Receiver<(Mesh, Explosion)>>,
) {
    for (mesh, explosion) in rx.try_iter() {
        spawn_explosion(
            commands,
            &particle_pipeline,
            &mut meshes,
            &mut materials,
            mesh,
            explosion.position,
            Timer::new(explosion.duration, false),
        );
    }
}

fn spawn_explosion(
    commands: &mut Commands,
    particle_pipeline: &Res<ParticlePipeline>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ParticleDirectionMaterial>>,
    mesh: Mesh,
    position: Vec3,
    timer: Timer,
) {
    commands
        .spawn(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                particle_pipeline.handle.as_weak(),
            )]),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(materials.add(ParticleDirectionMaterial { multiplier: 0.0 }))
        .with(ExplosionMarker)
        .with(timer);
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "b8ba5506-487a-4fed-88a2-c6bac4a22016"]
struct ParticleDirectionMaterial {
    pub multiplier: f32,
}

struct ExplosionMarker;

fn update_particle_direction(
    mut material: ResMut<Assets<ParticleDirectionMaterial>>,
    time: Res<Time>,
) {
    let handles: Vec<_> = material.ids().collect();
    for handle in handles.into_iter() {
        let m = material.get_mut(handle).unwrap();
        let old = m.multiplier;
        m.multiplier = old + time.delta_seconds() * 10.0;
    }
}

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Particle_Direction;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 2, binding = 0) uniform ParticleDirectionMaterial_multiplier {
    float multiplier;
};
void main() {
    gl_Position = ViewProj * Model * (vec4(Vertex_Position, 1.0) + vec4(Particle_Direction, 0.0) * multiplier);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
void main() {
    o_Target = vec4(1.0, 0.5, 0.5, 0.5);
}
"#;
