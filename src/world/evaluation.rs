use std::collections::HashSet;

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use flume::{Receiver, Sender};

use crate::{
    movement::model::UnitRotation,
    voxel_world::{
        access::VoxelAccess,
        chunk::{ChunkBoundaries, VoxelChunk},
    },
};

use super::{
    internal_model::FreeFloatingVoxel,
    model::{DelayedWorldTransformations, WorldUpdateEvent, WorldUpdateResult},
};

pub fn evaluate_delayed_transformations(
    mut effects_res: ResMut<DelayedWorldTransformations>,
    time: Res<Time>,
    mut update_events: ResMut<Events<WorldUpdateEvent>>,
) {
    let mut at_least_one = false;
    for (timer, effect) in effects_res.transformations.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
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
    mut voxel_chunk_query: Query<(&mut VoxelChunk,)>,
    tx: Res<Sender<WorldUpdateResult>>,
    chunk_access: Res<VoxelAccess>,
) {
    let mut changed = HashSet::new();
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
                if let Ok((mut chunk,)) = voxel_chunk_query.get_mut(entity) {
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
        if let Ok((foo,)) = voxel_chunk_query.get_mut(e) {
            entity_chunks.push((e, foo.clone()));
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
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rx: Res<Receiver<WorldUpdateResult>>,
) {
    if !rx.is_empty() {
        let texture = asset_server.load("world_texture_color.png");
        let material = materials.add(StandardMaterial {
            albedo_texture: Some(texture),
            ..Default::default()
        });

        for world_update_result in rx.try_iter() {
            for (entity, mesh) in world_update_result.entity_2_mesh {
                commands.set_current_entity(entity);
                commands
                    .remove::<(Handle<Mesh>,)>(entity)
                    .with(meshes.add(mesh));
            }
            for voxel in world_update_result.voxels_to_replace {
                let mesh = meshes.add(Mesh::from(&voxel));
                let bundle = PbrBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_translation(voxel.position.to_vec()),
                    ..Default::default()
                };
                commands
                    .spawn(bundle)
                    .with(FreeFloatingVoxel)
                    .with(UnitRotation {
                        rotation: Vec3::zero(),
                    });
            }
        }
    }
}
