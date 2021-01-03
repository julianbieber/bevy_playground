use crate::input::ReceivesInput;
use crate::movement::{MoveEvent, UnitRotation};

use crate::ai::model::{NPCBehaviours, NPC};
use bevy::prelude::*;
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
            };
            if rotation_offset.is_nan() {
                panic!()
            }
            movement_events.send(MoveEvent {
                rotation_offset,
                translation_offset: Vec3::new(0.0, 0.0, -1.0) * npc.velocity * time.delta_seconds(),
                entity: npc_entity,
            });
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
