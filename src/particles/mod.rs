mod mesh;
pub mod model;
mod primitives;
mod render;
mod spawn;

use bevy::{prelude::*, render::pipeline::RenderPipeline, tasks::AsyncComputeTaskPool};

use crate::delayed_despawn::DelayedDespawns;
use crate::particles::mesh::create_particle_mesh;
use crate::particles::model::{ParticleDescription, ParticleTypes};
use crate::particles::render::{
    setup_particles, update_particle_direction, ParticleDirectionMaterial, ParticlePipeline,
};
use crate::particles::spawn::{spawn_regular_explosions_system, ExplosionSpawnCoolDown};
use bevy::render::pipeline::PipelineDescriptor;
use flume::{unbounded, Receiver, Sender};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (tx, rx) = unbounded::<(Mesh, ParticleDescription)>();
        let mut timer = Timer::from_seconds(100.0, true);
        timer.tick(99.0);
        app.add_resource(ExplosionSpawnCoolDown { timer })
            .add_asset::<ParticleDirectionMaterial>()
            .add_resource(tx)
            .add_resource(rx)
            .init_resource::<DelayedParticleSpawns>()
            .add_startup_system(setup_particles.system())
            .add_system(spawn_regular_explosions_system.system())
            .add_system(update_particle_direction.system())
            .add_system(evaluate_delayed_particles.system())
            .add_system(spawn_from_channel.system());
    }
}

#[derive(Default)]
pub struct DelayedParticleSpawns {
    pub spawns: Vec<(Timer, ParticleDescription)>,
}

fn evaluate_delayed_particles(
    mut delayed_particle_spawns_res: ResMut<DelayedParticleSpawns>,
    time: Res<Time>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<(Mesh, ParticleDescription)>>,
) {
    let mut at_least_one = false;
    for (timer, explosion) in delayed_particle_spawns_res.spawns.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            let e = explosion.clone();
            let tx_cloned = tx.clone();
            pool.spawn(async move {
                let mesh = create_particle_mesh(&e);
                tx_cloned.send((mesh, e)).unwrap();
            })
            .detach();
            at_least_one = true;
        }
    }

    if at_least_one {
        let remaining: Vec<(Timer, ParticleDescription)> = delayed_particle_spawns_res
            .spawns
            .iter()
            .filter(|(t, _)| !t.just_finished())
            .map(|(t, e)| (t.clone(), e.clone()))
            .collect();

        delayed_particle_spawns_res.spawns = remaining;
    }
}

fn spawn_from_channel(
    commands: &mut Commands,
    particle_pipeline: Res<ParticlePipeline>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleDirectionMaterial>>,
    rx: Res<Receiver<(Mesh, ParticleDescription)>>,
    mut despanws_res: ResMut<DelayedDespawns>,
) {
    for (mesh, particles) in rx.try_iter() {
        let entity = spawn_explosion(
            commands,
            &mut meshes,
            &mut materials,
            mesh,
            particles.position,
            match particles.typ {
                ParticleTypes::Explosion { .. } => particle_pipeline.explosion_handle.as_weak(),
                ParticleTypes::HighStorm => particle_pipeline.high_storms_handle.as_weak(),
            },
            particles.typ,
        );
        despanws_res
            .despawns
            .push((Timer::new(particles.duration, false), entity));
    }
}

fn spawn_explosion(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ParticleDirectionMaterial>>,
    mesh: Mesh,
    position: Vec3,
    handle: Handle<PipelineDescriptor>,
    typ: ParticleTypes,
) -> Entity {
    commands
        .spawn(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(handle)]),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(materials.add(ParticleDirectionMaterial { multiplier: 0.0 }))
        .with(typ)
        .current_entity()
        .unwrap()
}
