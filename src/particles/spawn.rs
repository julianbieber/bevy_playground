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
use rand::{thread_rng, Rng};

use crate::particles::mesh::create_explosion_mesh;
use crate::particles::model::Explosion;
use flume::{unbounded, Receiver, Sender};
use std::time::Duration;

pub struct ExplosionSpawnCoolDown {
    pub timer: Timer,
}

pub fn spawn_regular_explosions_system(
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
