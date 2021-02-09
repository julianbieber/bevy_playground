use std::sync::{Arc, Mutex};

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::particles::model::ParticleTypes;
use crate::voxel_world::generator::VoxelWorld;
use crate::voxel_world::voxel::{Voxel, VoxelPosition};
use crate::{
    physics::collider::{Collider, ColliderShapes},
    voxel_world::world_structure::Terrain,
};
use ahash::AHashMap;
use flume::{unbounded, Receiver, Sender};
use rand::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (tx, rx) = unbounded::<(Mesh, Terrain, Option<Entity>)>();
        app.insert_resource(tx)
            .insert_resource(rx)
            .insert_resource(DelayedWorldTransformations {
                transformations: Vec::new(),
            })
            .add_event::<WorldUpdateEvent>()
            .add_system(update_world_from_channel.system())
            .add_system(update_world_event_reader.system())
            .add_system(erosion.system())
            .add_system(evaluate_delayed_transformations.system());
    }
}

pub fn world_setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<(Mesh, Terrain, Option<Entity>)>>,
) {
    let w = VoxelWorld::generate(150, 150, SmallRng::from_entropy());
    w.add_to_world(pool, tx);

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 100.0, 4.0)),
        ..Default::default()
    });

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.5 }));
    let cube_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.0, 1.0, 0.0),
        ..Default::default()
    });
    commands
        // parent cube
        .spawn(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 22.0, 0.1)),
            ..Default::default()
        })
        .with(Collider {
            collider_shape: ColliderShapes::Cuboid {
                half_width_x: 0.25,
                half_height_y: 0.25,
                half_depth_z: 0.25,
            },
            local_position: Vec3::new(0.0, 0.0, 0.0),
        });
}

pub fn update_world_from_channel(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rx: Res<Receiver<(Mesh, Terrain, Option<Entity>)>>,
) {
    if !rx.is_empty() {
        let texture = asset_server.load("world_texture_color.png");
        let material = materials.add(StandardMaterial {
            albedo_texture: Some(texture),
            ..Default::default()
        });

        for (mesh, terrain, optional_entity) in rx.try_iter() {
            if let Some(entity) = optional_entity {
                commands.set_current_entity(entity);
                commands
                    .remove::<(Handle<Mesh>, Terrain)>(entity)
                    .with(meshes.add(mesh))
                    .with(terrain);
            } else {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(mesh),
                        material: material.clone(),
                        ..Default::default()
                    })
                    .with(terrain);
            }
        }
    }
}

#[derive(Clone)]
pub struct WorldUpdateEvent {
    pub entity: Entity,
    pub delete: Arc<dyn Fn(&Terrain) -> Vec<VoxelPosition> + Send + Sync>,
}

pub fn update_world_event_reader(
    mut update_events: EventReader<WorldUpdateEvent>,
    terrain_query: Query<&Terrain>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<(Mesh, Terrain, Option<Entity>)>>,
) {
    let mut updates: AHashMap<
        Entity,
        Vec<Arc<dyn Fn(&Terrain) -> Vec<VoxelPosition> + Send + Sync>>,
    > = AHashMap::new();
    for event in update_events.iter() {
        updates
            .entry(event.entity)
            .or_insert(Vec::new())
            .push(event.delete.clone());
    }

    for (entity, updates) in updates.into_iter() {
        let mut old_terrain = terrain_query.get(entity).unwrap().clone();
        let tx_c = tx.clone();
        pool.0
            .spawn(async move {
                for voxel_fn in updates.into_iter() {
                    for voxel in voxel_fn(&old_terrain).into_iter() {
                        old_terrain.remove_voxel(voxel);
                    }
                }
                let new_mesh = Mesh::from(&old_terrain);
                tx_c.send((new_mesh, old_terrain, Some(entity)))
            })
            .detach();
    }
}

pub fn erosion(
    particle_emitters_query: Query<(&ParticleTypes, &Transform)>,
    mut update_events: ResMut<Events<WorldUpdateEvent>>,
) {
    for (particle_type, transform) in particle_emitters_query.iter() {
        match particle_type {
            ParticleTypes::Explosion { radius } => {}
            ParticleTypes::HighStorm { depth } => {}
        }
    }
}

pub struct DelayedWorldTransformations {
    pub transformations: Vec<(Timer, WorldUpdateEvent)>,
}

fn evaluate_delayed_transformations(
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
