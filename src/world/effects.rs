use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    movement::model::MoveEvent,
    particles::model::ParticleTypes,
    voxel_world::{voxel::VoxelPosition, world_structure::Terrain},
};

use super::{internal_model::FreeFloatingVoxel, model::WorldUpdateEvent};
use rand::prelude::*;

pub fn erosion(
    particle_emitters_query: Query<(&ParticleTypes, &Transform)>,
    mut update_events: ResMut<Events<WorldUpdateEvent>>,
    world_query: Query<(Entity, &Terrain)>,
) {
    for (particle_type, transform) in particle_emitters_query.iter() {
        match particle_type {
            ParticleTypes::Explosion { radius } => {}
            ParticleTypes::HighStorm { depth } => {
                for (terrain_entity, _) in world_query.iter() {
                    let highstorm_center = transform.translation.clone();
                    let d = depth.clone();
                    let mut rng = SmallRng::from_entropy();
                    if rng.gen_range(0.0..1.0) < 0.01 {
                        let delete = Arc::new(move |terrain: &Terrain| {
                            let mut rng = SmallRng::from_entropy();
                            let p = VoxelPosition {
                                x: rng.gen_range(terrain.min[0]..terrain.max[0]),
                                y: terrain.max[1],
                                z: rng.gen_range(terrain.min[2]..terrain.max[2]),
                            };
                            let world_space = p.to_vec();
                            if world_space.x > highstorm_center.x - d
                                && world_space.x < highstorm_center.x + d
                                && world_space.y > highstorm_center.y
                                && world_space.y < highstorm_center.y + 60.0
                                && world_space.z > highstorm_center.z - 200.0
                                && world_space.z < highstorm_center.z + 200.0
                            {
                                vec![p]
                            } else {
                                Vec::new()
                            }
                        });

                        update_events.send(WorldUpdateEvent {
                            delete,
                            entity: terrain_entity,
                            replace: true,
                        });
                    }
                }
            }
        }
    }
}

pub fn move_floating_voxels(
    voxel_query: Query<(Entity, &FreeFloatingVoxel, &Transform)>,
    storm_query: Query<(&ParticleTypes, &Transform)>,
    mut movement_events: ResMut<Events<MoveEvent>>,
    time: Res<Time>,
) {
    let mut rng = SmallRng::from_entropy();
    for (particle_type, transform) in storm_query.iter() {
        match particle_type {
            ParticleTypes::Explosion { .. } => {}
            ParticleTypes::HighStorm { depth } => {
                let highstorm_center = transform.translation;
                for (voxel_entity, _, voxel_transform) in voxel_query.iter() {
                    let translation = voxel_transform.translation;
                    if translation.x > highstorm_center.x - depth
                        && translation.x < highstorm_center.x + depth
                        && translation.y > highstorm_center.y
                        && translation.y < highstorm_center.y + 60.0
                        && translation.z > highstorm_center.z - 200.0
                        && translation.z < highstorm_center.z + 200.0
                    {
                        movement_events.send(MoveEvent {
                            rotation_offset: Vec3::new(
                                rng.gen_range(-0.1..0.1),
                                rng.gen_range(-0.1..0.1),
                                rng.gen_range(-0.1..0.1),
                            ) * time.delta_seconds(),
                            translation_offset: Vec3::new(
                                rng.gen_range(-0.1..0.0) * 10.0,
                                rng.gen_range(-0.5..1.0) * 10.0,
                                rng.gen_range(-0.1..0.1) * 10.0,
                            ) * time.delta_seconds(),
                            entity: voxel_entity,
                        })
                    }
                }
            }
        }
    }
}
