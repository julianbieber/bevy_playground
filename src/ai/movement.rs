use crate::delayed_despawn::DelayedDespawns;
use crate::movement::model::{MoveEvent, UnitRotation};
use crate::particles::model::ParticleDescription;
use crate::particles::DelayedParticleSpawns;
use crate::player::model::ReceivesInput;
use crate::{
    ai::model::{NPCBehaviours, NPC},
    unit_effects::{DelayedEffects, Effect, Effects},
};
use bevy::prelude::*;
use bevy::utils::Duration;
use rand::prelude::*;

pub fn npc_movement_system(
    npcs_query: Query<(Entity, &NPC, &Transform, &UnitRotation)>,
    player_query: Query<(&ReceivesInput, &Transform)>,
    mut movement_events: ResMut<Events<MoveEvent>>,
    time: Res<Time>,
) {
    let mut rng = SmallRng::from_entropy();
    for (_, player_transform) in player_query.iter() {
        for (npc_entity, npc, npc_transform, current_rotation) in npcs_query.iter() {
            let rotation_offset = match npc.behaviour {
                NPCBehaviours::FOLLOW => {
                    rotation_toward(&player_transform.translation, &npc_transform.translation)
                        - current_rotation.rotation
                }
                NPCBehaviours::RANDOM => Vec3::new(
                    rng.gen_range(-0.1f32..0.1f32),
                    rng.gen_range(-0.1f32..0.1f32),
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
    mut effects_res: ResMut<DelayedEffects>,
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
                            ParticleDescription::explosion(
                                10.0,
                                Duration::from_secs(10),
                                10000,
                                npc_transform.translation,
                            ),
                        ));
                        despanws_res
                            .despawns
                            .push((Timer::from_seconds(2.1, false), entity));
                        effects_res.effects.push((
                            Timer::from_seconds(2.1, false),
                            Effect {
                                range: 10.0,
                                center: npc_transform.translation,
                                typ: Effects::Damage { amount: 10.0 },
                            },
                        ))
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
