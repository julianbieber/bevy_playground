use bevy::{app::Events, prelude::*, tasks::AsyncComputeTaskPool};
use common::{MoveEvent, ParticleTypes};

use crate::particles::mesh::create_particle_mesh;
use crate::particles::model::ParticleDescription;
use flume::Sender;
use std::time::Duration;

pub struct ExplosionSpawnCoolDown {
    pub timer: Timer,
}

pub fn spawn_regular_explosions_system(
    mut spawn_timer: ResMut<ExplosionSpawnCoolDown>,
    time: Res<Time>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<(Mesh, ParticleDescription)>>,
) {
    if spawn_timer.timer.tick(time.delta()).just_finished() {
        spawn_timer.timer.reset();
        let tx_copy = tx.clone();
        pool.0
            .spawn(async move {
                let e = ParticleDescription::high_storm(
                    Duration::from_secs(50),
                    500000,
                    Vec3::new(100.0, 0.0, 0.0),
                    50.0,
                );
                let mesh = create_particle_mesh(&e);
                tx_copy.send((mesh, e)).unwrap();
            })
            .detach();
    }
}

pub fn move_particle_emitters(
    particle_emitters_query: Query<(Entity, &ParticleTypes)>,
    mut movement_events: ResMut<Events<MoveEvent>>,
    time: Res<Time>,
) {
    for (e, p) in particle_emitters_query.iter() {
        match p {
            ParticleTypes::Explosion { .. } => {}
            ParticleTypes::HighStorm { .. } => movement_events.send(MoveEvent {
                entity: e,
                rotation_offset: Vec3::ZERO,
                translation_offset: Vec3::new(-10.0, 0.0, 0.0) * time.delta_seconds(),
                is_player: false,
            }),
        }
    }
}
