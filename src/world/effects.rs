use std::sync::Arc;

use bevy::prelude::*;
use itertools::Chunk;

use crate::{
    movement::model::MoveEvent,
    particles::model::ParticleTypes,
    voxel_world::{
        chunk::{ChunkBoundaries, VoxelChunk},
        voxel::VoxelPosition,
    },
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
                let mut rng = SmallRng::from_entropy();
                if rng.gen_range(0.0..1.0) < 0.01 {
                    let highstorm_center = transform.translation.clone();
                    let highstorm_center_voxel = VoxelPosition::from_vec3(&highstorm_center);

                    let d = depth.clone();
                    let d_voxel = VoxelPosition::voxel_distance(d);
                    let boundaries = ChunkBoundaries {
                        min: [
                            highstorm_center_voxel.x - 10,
                            highstorm_center_voxel.y - 100,
                            highstorm_center_voxel.z - d_voxel / 2,
                        ],
                        max: [
                            highstorm_center_voxel.x + 10,
                            highstorm_center_voxel.y + 100,
                            highstorm_center_voxel.z + d_voxel / 2,
                        ],
                    };
                    let boundaries_clone = boundaries.clone();
                    let delete = Arc::new(move |chunks: &Vec<VoxelChunk>| {
                        select_a_highest_voxel(&boundaries_clone, chunks)
                    });

                    update_events.send(WorldUpdateEvent {
                        chunk_filter: vec![boundaries],
                        delete,
                        replace: true,
                    });
                }
            }
        }
    }
}

fn select_a_highest_voxel(
    storm_boundaries: &ChunkBoundaries,
    chunks: &Vec<VoxelChunk>,
) -> Vec<VoxelPosition> {
    let voxels: Vec<_> = chunks
        .iter()
        .flat_map(|c| c.get_voxels().iter())
        .filter(|v| storm_boundaries.contains(&v.position))
        .collect();

    if let Some(highest) = voxels.iter().map(|v| v.position.y).max() {
        let highest_voxels: Vec<_> = voxels
            .into_iter()
            .filter(|v| v.position.y == highest)
            .collect();
        let mut rng = SmallRng::from_entropy();
        let position = rng.gen_range(0..highest_voxels.len());
        vec![highest_voxels[position].position.clone()]
    } else {
        vec![]
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
