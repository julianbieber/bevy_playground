use crate::ai::model::{NPCBehaviours, NPC};
use crate::delayed_despawn::DelayedDespawns;
use crate::movement::model::{MoveEvent, UnitRotation};
use crate::particles::model::{ParticleDescription, ParticleTypes};
use crate::particles::DelayedParticleSpawns;
use crate::player::model::ReceivesInput;
use bevy::prelude::*;
use bevy::utils::Duration;
use rand::prelude::ThreadRng;
use rand::Rng;

pub fn npc_movement_system(
    npcs_query: Query<(Entity, &NPC, &Transform, &UnitRotation)>,
    player_query: Query<(&ReceivesInput, &Transform)>,
    mut movement_events: ResMut<Events<MoveEvent>>,
    time: Res<Time>,
) {
    for (_, player_transform) in player_query.iter() {
        for (npc_entity, npc, npc_transform, current_rotation) in npcs_query.iter() {
            let rotation_offset = match npc.behaviour {
                NPCBehaviours::FOLLOW => {
                    rotation_toward(&player_transform.translation, &npc_transform.translation)
                        - current_rotation.rotation
                }
                NPCBehaviours::RANDOM => Vec3::new(
                    ThreadRng::default().gen_range(-0.1f32, 0.1f32),
                    ThreadRng::default().gen_range(-0.1f32, 0.1f32),
                    0.0,
                ),
                NPCBehaviours::EXPLODE => Vec3::zero(),
            };
            if let Some(forward) = match npc.behaviour {
                NPCBehaviours::EXPLODE => None,
                _ => Some(Vec3::new(0.0, 0.0, -1.0) * npc.velocity * time.delta_seconds()),
            } {
                if !rotation_offset.is_nan() {
                    movement_events.send(MoveEvent {
                        rotation_offset,
                        translation_offset: forward,
                        entity: npc_entity,
                    });
                }
            }
        }
    }
}

pub fn update_behaviour_system(
    mut npcs_query: Query<(Entity, &mut NPC, &Transform)>,
    player_query: Query<(&ReceivesInput, &Transform)>,
    mut delayed_spawn_res: ResMut<DelayedParticleSpawns>,
    mut despanws_res: ResMut<DelayedDespawns>,
) {
    for (_, player_transform) in player_query.iter() {
        for (entity, mut npc, npc_transform) in npcs_query.iter_mut() {
            match npc.behaviour {
                NPCBehaviours::FOLLOW => {
                    let distance_sq = player_transform
                        .translation
                        .distance_squared(npc_transform.translation);
                    if distance_sq < 50.0f32 {
                        npc.behaviour = NPCBehaviours::EXPLODE;
                        delayed_spawn_res.spawns.push((
                            Timer::from_seconds(2.0, false),
                            ParticleDescription {
                                typ: ParticleTypes::Explosion { radius: 10.0 },
                                duration: Duration::from_secs(10),
                                particles: 10000,
                                position: npc_transform.translation,
                            },
                        ));
                        despanws_res
                            .despawns
                            .push((Timer::from_seconds(2.1, false), entity))
                    }
                }
                NPCBehaviours::RANDOM => {
                    let distance_sq = player_transform
                        .translation
                        .distance_squared(npc_transform.translation);
                    if distance_sq < 10000.0f32 {
                        npc.behaviour = NPCBehaviours::FOLLOW;
                    }
                }
                NPCBehaviours::EXPLODE => {}
            }
        }
    }
}

fn rotation_toward(target: &Vec3, object: &Vec3) -> Vec3 {
    let object_in_target_system = (*object - (target.clone())).normalize();

    let angle1 = -1.0 * object_in_target_system.z.atan2(object_in_target_system.x)
        + std::f32::consts::PI / 2.0;
    let angle2 = -1.0 * object_in_target_system.y.asin();

    Vec3::new(angle1, angle2, 0.0)
}
