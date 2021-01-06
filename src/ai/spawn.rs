use crate::ai::model::*;
use bevy::prelude::*;

use crate::movement::model::{Movable, UnitRotation};
use rand::{thread_rng, Rng};

pub struct SpawnCoolDown {
    pub timer: Timer,
}

pub fn enemy_spawn_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cooldown: ResMut<SpawnCoolDown>,
    time: Res<Time>,
) {
    if cooldown.timer.tick(time.delta_seconds()).just_finished() {
        let mut thread_rng = thread_rng();
        cooldown.timer.reset();
        cooldown
            .timer
            .set_duration(thread_rng.gen_range(0.5f32, 2.0f32));

        let cube_handle = meshes.add(Mesh::from(shape::Cube {
            size: thread_rng.gen_range(0.5f32, 5.0f32),
        }));
        let cube_material_handle = materials.add(StandardMaterial {
            albedo: Color::rgb(1.0, 0.0, thread_rng.gen_range(0.0f32, 1.0f32)),
            ..Default::default()
        });

        commands
            // parent cube
            .spawn(PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                transform: Transform::from_translation(Vec3::new(
                    thread_rng.gen_range(-100.0f32, 100.0f32),
                    thread_rng.gen_range(0.0f32, 100.0f32),
                    thread_rng.gen_range(-100.0f32, 100.0f32),
                )),
                ..Default::default()
            })
            .with(NPC {
                behaviour: NPCBehaviours::RANDOM,
                velocity: thread_rng.gen_range(1.0f32, 5.0f32),
            })
            .with(Movable)
            .with(UnitRotation {
                ..Default::default()
            });
    }
}
