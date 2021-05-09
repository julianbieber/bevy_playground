use bevy::{app::Events, prelude::*, tasks::AsyncComputeTaskPool};
use flume::{Receiver, Sender};

use crate::{
    movement::model::UnitRotation,
    voxel_world::{access::VoxelAccess, boundaries::ChunkBoundaries, chunk::VoxelChunk},
};

use super::VoxelTexture;
use super::{
    internal_model::FreeFloatingVoxel,
    model::{DelayedWorldTransformations, WorldUpdateEvent, WorldUpdateResult},
};
use crate::player::PlayerPosition;
use crate::voxel_world::distance_2_lod;
use ahash::AHashSet;
use bevy::render::mesh::Indices;

pub fn evaluate_delayed_transformations(
    mut effects_res: ResMut<DelayedWorldTransformations>,
    time: Res<Time>,
    mut update_events: ResMut<Events<WorldUpdateEvent>>,
) {
    let mut at_least_one = false;
    for (timer, effect) in effects_res.transformations.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            at_least_one = true;
            update_events.send((*effect).clone())
        }
    }

    if at_least_one {
        let remaining: Vec<(Timer, WorldUpdateEvent)> = effects_res
            .transformations
            .iter()
            .filter(|(t, _)| !t.just_finished())
            .map(|(t, e)| (t.clone(), (*e).clone()))
            .collect();

        effects_res.transformations = remaining;
    }
}

pub fn update_world_event_reader(
    mut update_events: EventReader<WorldUpdateEvent>,
    pool: ResMut<AsyncComputeTaskPool>,
    mut voxel_chunk_query: Query<(Entity, &mut VoxelChunk)>,
    tx: Res<Sender<WorldUpdateResult>>,
    chunk_access: Res<VoxelAccess>,
    player_position: Res<PlayerPosition>,
) {
    let mut changed = AHashSet::new();

    for (entity, mut voxel_chunk) in voxel_chunk_query.iter_mut() {
        let center: Vec3 = voxel_chunk.boundary.center().to_vec();
        let lod = distance_2_lod(center.distance(player_position.position));
        if lod != voxel_chunk.lod {
            voxel_chunk.lod = lod;
            changed.insert(entity);
        }
    }

    let mut replaces = Vec::new();

    for event in update_events.iter() {
        let filtered_chunks: Vec<_> = event
            .chunk_filter
            .iter()
            .flat_map(|boundary| ChunkBoundaries::aligned_boundaries_in(boundary))
            .flat_map(|b| {
                chunk_access
                    .get_chunk(&b, &mut voxel_chunk_query)
                    .into_iter()
            })
            .collect();

        let deletes = (event.delete)(&filtered_chunks);
        for delete in deletes {
            if let Some(entity) = chunk_access.get_chunk_entity_containing(delete) {
                if let Ok((_, mut chunk)) = voxel_chunk_query.get_mut(entity) {
                    if let Some(voxel) = chunk.remove(delete) {
                        if event.replace {
                            replaces.push(voxel);
                        }
                        changed.insert(entity);
                    }
                }
            }
        }
    }

    let mut entity_chunks = Vec::with_capacity(changed.len());
    for e in changed {
        if let Ok((_, chunk)) = voxel_chunk_query.get_mut(e) {
            entity_chunks.push((e, chunk.clone()));
        }
    }

    let tx_c = tx.clone();
    pool.0
        .spawn(async move {
            let entity_mesh: Vec<_> = entity_chunks
                .into_iter()
                .map(|(e, chunk)| {
                    let mesh = Mesh::from(&chunk);
                    (e, mesh)
                })
                .collect();

            tx_c.send(WorldUpdateResult {
                entity_2_mesh: entity_mesh,
                voxels_to_replace: replaces,
            })
        })
        .detach();
}

pub fn update_world_from_channel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<VoxelTexture>,
    rx: Res<Receiver<WorldUpdateResult>>,
) {
    for world_update_result in rx.try_iter() {
        for (entity, mesh) in world_update_result.entity_2_mesh {
            if match mesh.indices().unwrap() {
                Indices::U16(i) => i.len() != 0,
                Indices::U32(i) => i.len() != 0,
            } {
                commands
                    .entity(entity)
                    .remove::<(Handle<Mesh>,)>()
                    .insert(meshes.add(mesh));
            } else {
                commands.entity(entity).remove_bundle::<PbrBundle>();
            }
        }
        for voxel in world_update_result.voxels_to_replace {
            let mesh = meshes.add(Mesh::from(&voxel));
            let bundle = PbrBundle {
                mesh,
                material: material.material.clone(),
                transform: Transform::from_translation(voxel.position.to_vec()),
                ..Default::default()
            };
            commands
                .spawn_bundle(bundle)
                .insert(FreeFloatingVoxel)
                .insert(UnitRotation {
                    rotation: Vec3::ZERO,
                });
        }
    }
}
