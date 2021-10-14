#[macro_use]
extern crate smallvec;

pub mod boundaries;
pub mod collision;
mod effects;
mod evaluation;
pub mod generator;
mod lod;
mod mesh;
mod meshing;
pub mod model;
pub mod prelude;
pub mod voxel;
pub mod water;
mod world_gen;
pub mod world_sector;

use ahash::AHashMap;
use bevy::prelude::Plugin;
use bevy::prelude::*;
use bevy_collision::collider::{Collider, ColliderShapes};
use boundaries::{ChunkBoundaries, CHUNK_SIZE};
use world_sector::{DefaultWorldSector};

use crate::voxel::Voxel;
use flume::unbounded;
use model::{DelayedWorldTransformations, WorldUpdateResult};

pub struct AdditionalVoxels {
    voxels: AHashMap<ChunkBoundaries<CHUNK_SIZE>, Vec<Voxel>>,
}

pub struct FreeFloatingVoxel;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (tx, rx) = unbounded::<WorldUpdateResult>();

        let mut world_sector = {
            let mut w = DefaultWorldSector::new([0, 0, 0].into());
            w.insert_terrain();
            w
        };
        app.insert_resource(tx)
            .insert_resource(rx)
            .insert_resource(world_sector)
            .insert_resource(DelayedWorldTransformations {
                transformations: Vec::new(),
            })
            .add_startup_system(meshing::initialize_system.system())
            .add_startup_system(world_setup.system());
        /*.add_event::<WorldUpdateEvent>()
        .add_system(update_world_from_channel.system())
        .add_system(update_world_event_reader.system())
        .add_system(erosion.system())
        .add_system(evaluate_delayed_transformations.system())
        .add_system(move_floating_voxels.system())
        .add_startup_system(world_setup.system())
        .add_startup_system(setup_world_gen.system())
        .add_system(start_generation.system())
        .add_system(read_generation_results.system());
        */
    }
}

fn world_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
        point_light: PointLight {
            intensity: 100000.0,
            range: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.5 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::hex("00ff00").unwrap(),
        ..Default::default()
    });
    commands
        // parent cube
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 22.0, 0.1)),
            ..Default::default()
        })
        .insert(Collider {
            collider_shape: ColliderShapes::Cuboid {
                half_width_x: 0.25,
                half_height_y: 0.25,
                half_depth_z: 0.25,
            },
            local_position: Vec3::new(0.0, 0.0, 0.0),
        });
}
