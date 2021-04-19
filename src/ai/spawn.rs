use std::time::Duration;

use crate::ai::model::*;
use bevy::prelude::*;

use crate::movement::model::{Movable, UnitRotation};
use rand::prelude::*;

pub struct SpawnCoolDown {
    pub timer: Timer,
}

pub fn enemy_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cooldown: ResMut<SpawnCoolDown>,
    npc_query: Query<(&NPC,)>,
    time: Res<Time>,
) {
    if cooldown.timer.tick(time.delta()).just_finished() {
        if npc_query.iter().count() < 10 {
            let mut rng = SmallRng::from_entropy();
            cooldown.timer.reset();
            cooldown
                .timer
                .set_duration(Duration::from_millis(rng.gen_range(500..2000)));

            let cube_handle = meshes.add(Mesh::from(shape::Cube {
                size: rng.gen_range(0.5f32..5.0f32),
            }));
            let cube_material_handle = materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.0, rng.gen_range(0.0f32..1.0f32)),
                ..Default::default()
            });

            commands
                // parent cube
                .spawn_bundle(PbrBundle {
                    mesh: cube_handle,
                    material: cube_material_handle,
                    transform: Transform::from_translation(Vec3::new(
                        rng.gen_range(-100.0f32..100.0f32),
                        rng.gen_range(0.0f32..100.0f32),
                        rng.gen_range(-100.0f32..100.0f32),
                    )),
                    ..Default::default()
                })
                .insert(NPC {
                    behaviour: NPCBehaviours::RANDOM,
                    velocity: rng.gen_range(1.0f32..5.0f32),
                })
                .insert(Movable)
                .insert(UnitRotation {
                    ..Default::default()
                });
        }
    }
}
