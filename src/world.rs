use std::sync::{Arc, Mutex};

use bevy::{prelude::*, tasks::AsyncComputeTaskPool, transform};

use crate::{delayed_despawn, particles::model::ParticleTypes};
use crate::{
    movement::model::MoveEvent,
    unit_effects::DelayedEffects,
    voxel_world::voxel::{Voxel, VoxelPosition},
};
use crate::{
    movement::model::UnitRotation,
    voxel_world::{self, generator::VoxelWorld},
};
use crate::{
    physics::collider::{Collider, ColliderShapes},
    voxel_world::voxel::world_2_voxel_space,
    voxel_world::world_structure::Terrain,
};
use ahash::AHashMap;
use flume::{unbounded, Receiver, Sender};
use rand::prelude::*;

pub struct WorldPlugin;
pub struct FreeFloatingVoxel;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (tx, rx) = unbounded::<WorldUpdateResult>();
        app.insert_resource(tx)
            .insert_resource(rx)
            .insert_resource(DelayedWorldTransformations {
                transformations: Vec::new(),
            })
            .add_event::<WorldUpdateEvent>()
            .add_system(update_world_from_channel.system())
            .add_system(update_world_event_reader.system())
            .add_system(erosion.system())
            .add_system(evaluate_delayed_transformations.system())
            .add_system(move_floating_voxels.system());
    }
}

pub fn world_setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<WorldUpdateResult>>,
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

            // TODO
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

#[derive(Clone)]
pub struct WorldUpdateEvent {
    pub entity: Entity,
    pub delete: Arc<dyn Fn(&Terrain) -> Vec<VoxelPosition> + Send + Sync>,
    pub replace: bool,
}

pub struct WorldUpdateResult {
    pub new_terrain_mesh: Mesh,
    pub terrain: Terrain,
    pub existing_terrain_entity: Option<Entity>,
    pub voxels_to_replace: Vec<Voxel>,
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
                                rng.gen_range(-0.1..0.0),
                                rng.gen_range(-0.5..1.0),
                                rng.gen_range(-0.1..0.1),
                            ) * time.delta_seconds(),
                            entity: voxel_entity,
                        })
                    }
                }
            }
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
