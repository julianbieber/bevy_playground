mod effects;
mod evaluation;
mod internal_model;
pub mod model;

use ahash::AHashMap;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::voxel_world::{
    access::VoxelAccess, chunk::ChunkBoundaries, generator::VoxelWorld, voxel::Voxel,
};
use crate::{
    physics::collider::{Collider, ColliderShapes},
    voxel_world::{chunk::VoxelChunk, chunk_mesh},
};
use flume::{unbounded, Sender};
use rand::prelude::*;

use self::{
    effects::{erosion, move_floating_voxels},
    evaluation::{
        evaluate_delayed_transformations, update_world_event_reader, update_world_from_channel,
    },
    model::{DelayedWorldTransformations, WorldUpdateEvent, WorldUpdateResult},
};

pub struct WorldPlugin;

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
            .add_system(move_floating_voxels.system())
            .add_startup_system(world_setup.system());
    }
}

fn world_setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<WorldUpdateResult>>,
    asset_server: Res<AssetServer>,
    mut chunk_access: ResMut<VoxelAccess>,
) {
    let w = VoxelWorld::generate(150, 150, SmallRng::from_entropy());
    let mut chunk_map = AHashMap::new();
    for pillar in w.pillars {
        for voxel in pillar.voxels() {
            let matching_boundary = ChunkBoundaries::aligned(voxel.position);
            chunk_map
                .entry(matching_boundary)
                .or_insert(VoxelChunk::empty())
                .set(voxel);
        }
    }
    let chunk_texture = asset_server.load("world_texture_color.png");
    for (boundary, chunk) in chunk_map {
        let chunk_mesh = meshes.add(Mesh::from(&chunk));

        let chunk_material = materials.add(StandardMaterial {
            albedo_texture: Some(chunk_texture.clone()),
            ..Default::default()
        });
        let chunk_bundle = PbrBundle {
            mesh: chunk_mesh,
            material: chunk_material.clone(),
            transform: Transform::from_translation(Vec3::zero()),
            ..Default::default()
        };
        let chunk_entity = commands
            .spawn(chunk_bundle)
            .with(chunk)
            .current_entity()
            .unwrap();
        chunk_access.add_chunk(boundary, chunk_entity);
    }

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
