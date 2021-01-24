use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use rand::{thread_rng, Rng};

use crate::particles::mesh::create_particle_mesh;
use crate::particles::model::{ParticleDescription, ParticleTypes};
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
    if spawn_timer.timer.tick(time.delta_seconds()).just_finished() {
        spawn_timer.timer.reset();
        let tx_copy = tx.clone();
        pool.0
            .spawn(async move {
                let e = ParticleDescription::high_storm(
                    Duration::from_secs(50),
                    500000,
                    Vec3::new(100.0, 0.0, 0.0),
                );
                let mesh = create_particle_mesh(&e);
                tx_copy.send((mesh, e)).unwrap();
            })
            .detach();
    }
}
