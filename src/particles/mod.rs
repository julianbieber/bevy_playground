mod mesh;
pub mod model;
mod primitives;
mod render;
mod spawn;

use bevy::{
    prelude::*,
    render::{
        pipeline::RenderPipeline,
    },
    tasks::AsyncComputeTaskPool,
};

use crate::delayed_despawn::DelayedDespawns;
use crate::particles::mesh::create_explosion_mesh;
use crate::particles::model::Explosion;
use crate::particles::render::{
    setup_particles, update_particle_direction, ParticleDirectionMaterial, ParticlePipeline,
};
use crate::particles::spawn::{spawn_regular_explosions_system, ExplosionSpawnCoolDown};
use flume::{unbounded, Receiver, Sender};


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
        .add_system(update_particle_direction.system())
        .add_system(evaluate_delayed_particles.system())
        .add_system(spawn_from_channel.system());
    }
}

#[derive(Default)]
pub struct DelayedParticleSpawns {
    pub spawns: Vec<(Timer, Explosion)>,
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

fn spawn_from_channel(
    commands: &mut Commands,
    particle_pipeline: Res<ParticlePipeline>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleDirectionMaterial>>,
    rx: Res<Receiver<(Mesh, Explosion)>>,
    mut despanws_res: ResMut<DelayedDespawns>,
) {
    for (mesh, explosion) in rx.try_iter() {
        let entity = spawn_explosion(
            commands,
            &particle_pipeline,
            &mut meshes,
            &mut materials,
            mesh,
            explosion.position,
        );
        despanws_res
            .despawns
            .push((Timer::new(explosion.duration, false), entity));
    }
}

fn spawn_explosion(
    commands: &mut Commands,
    particle_pipeline: &Res<ParticlePipeline>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ParticleDirectionMaterial>>,
    mesh: Mesh,
    position: Vec3,
) -> Entity {
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
        .current_entity()
        .unwrap()
}
