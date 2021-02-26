use std::sync::Arc;

use ahash::AHashMap;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use flume::{Receiver, Sender};

use crate::{
    movement::model::UnitRotation,
    voxel_world::{
        voxel::{Voxel, VoxelPosition},
        world_structure::Terrain,
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
    terrain_query: Query<&Terrain>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<WorldUpdateResult>>,
) {
    let mut updates: AHashMap<
        Entity,
        Vec<(
            Arc<dyn Fn(&Terrain) -> Vec<VoxelPosition> + Send + Sync>,
            bool,
        )>,
    > = AHashMap::new();
    for event in update_events.iter() {
        updates
            .entry(event.entity)
            .or_insert(Vec::new())
            .push((event.delete.clone(), event.replace));
    }

    for (entity, updates) in updates.into_iter() {
        let mut old_terrain = terrain_query.get(entity).unwrap().clone();
        let tx_c = tx.clone();
        pool.0
            .spawn(async move {
                let mut replace_voxels: Vec<Voxel> = Vec::new();
                for (voxel_fn, replace) in updates.into_iter() {
                    for voxel in voxel_fn(&old_terrain).into_iter() {
                        old_terrain.remove_voxel(voxel).map(|v| {
                            if replace {
                                replace_voxels.push(v);
                            }
                        });
                    }
                }
                old_terrain.recalculate();
                let new_mesh = Mesh::from(&old_terrain);
                tx_c.send(WorldUpdateResult {
                    new_terrain_mesh: new_mesh,
                    terrain: old_terrain,
                    existing_terrain_entity: Some(entity),
                    voxels_to_replace: replace_voxels,
                })
            })
            .detach();
    }
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
            if let Some(entity) = world_update_result.existing_terrain_entity {
                commands.set_current_entity(entity);
                commands
                    .remove::<(Handle<Mesh>, Terrain)>(entity)
                    .with(meshes.add(world_update_result.new_terrain_mesh))
                    .with(world_update_result.terrain);
            } else {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(world_update_result.new_terrain_mesh),
                        material: material.clone(),
                        ..Default::default()
                    })
                    .with(world_update_result.terrain);
            }

            for voxel in world_update_result.voxels_to_replace.iter() {
                let mesh = meshes.add(Mesh::from(voxel));
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
