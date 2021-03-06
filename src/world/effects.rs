use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    movement::model::MoveEvent, particles::model::ParticleTypes, voxel_world::voxel::VoxelPosition,
};

use super::{internal_model::FreeFloatingVoxel, model::WorldUpdateEvent};
use rand::prelude::*;

pub fn erosion(
    particle_emitters_query: Query<(&ParticleTypes, &Transform)>,
    mut update_events: ResMut<Events<WorldUpdateEvent>>,
) {
    for (particle_type, transform) in particle_emitters_query.iter() {
        match particle_type {
            ParticleTypes::Explosion { radius: _ } => {}
            ParticleTypes::HighStorm { depth } => {
                let highstorm_center = transform.translation.clone();
                let d = depth.clone();
                let mut rng = SmallRng::from_entropy();
                if rng.gen_range(0.0..1.0) < 0.01 {
                    let delete = Arc::new(move || {
                        // todo get highest voxel at x|z
                        Vec::new()
                    });

                    update_events.send(WorldUpdateEvent {
                        delete,
                        replace: true,
                    });
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
