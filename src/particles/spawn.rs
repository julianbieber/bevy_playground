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
                let e = ParticleDescription {
                    typ: ParticleTypes::Explosion { radius: 10.0 },
                    duration: Duration::from_secs(2),
                    particles: 10000,
                    position: Vec3::new(
                        thread_rng().gen_range(-100.0f32, 100.0f32),
                        thread_rng().gen_range(0.0f32, 100.0f32),
                        thread_rng().gen_range(-100.0f32, 100.0f32),
                    ),
                };
                let mesh = create_particle_mesh(&e);
                tx_copy.send((mesh, e)).unwrap();
            })
            .detach();
    }
}
