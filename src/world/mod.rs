mod effects;
mod evaluation;
mod internal_model;
pub mod model;

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::voxel_world::generator::VoxelWorld;
use crate::{
    physics::collider::{Collider, ColliderShapes},
    voxel_world::{
        chunk::VoxelChunk,
        chunk_mesh,
        voxel::{Voxel, VoxelPosition, VoxelTypes},
    },
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
) {
    let w = VoxelWorld::generate(150, 150, SmallRng::from_entropy());
    w.add_to_world(pool, tx);

    let mut test_chunk = VoxelChunk::empty();
    test_chunk.set(Voxel {
        position: VoxelPosition { x: 0, y: 50, z: 0 },
        typ: VoxelTypes::DarkRock1,
    });
    test_chunk.set(Voxel {
        position: VoxelPosition { x: 0, y: 51, z: 0 },
        typ: VoxelTypes::DarkRock1,
    });
    test_chunk.set(Voxel {
        position: VoxelPosition { x: 0, y: 49, z: 0 },
        typ: VoxelTypes::DarkRock1,
    });

    test_chunk.set(Voxel {
        position: VoxelPosition { x: -1, y: 50, z: 0 },
        typ: VoxelTypes::DarkRock1,
    });
    test_chunk.set(Voxel {
        position: VoxelPosition { x: -1, y: 51, z: 0 },
        typ: VoxelTypes::DarkRock1,
    });
    test_chunk.set(Voxel {
        position: VoxelPosition { x: -1, y: 49, z: 0 },
        typ: VoxelTypes::DarkRock1,
    });

    test_chunk.set(Voxel {
        position: VoxelPosition { x: 1, y: 50, z: 0 },
        typ: VoxelTypes::DarkRock1,
    });
    test_chunk.set(Voxel {
        position: VoxelPosition { x: 1, y: 51, z: 0 },
        typ: VoxelTypes::DarkRock1,
    });
    test_chunk.set(Voxel {
        position: VoxelPosition { x: 1, y: 49, z: 0 },
        typ: VoxelTypes::DarkRock1,
    });

    let chunk_mesh = meshes.add(Mesh::from(&test_chunk));

    let chunk_texture = asset_server.load("world_texture_color.png");
    let chunk_material = materials.add(StandardMaterial {
        albedo_texture: Some(chunk_texture),
        ..Default::default()
    });
    let chunk_bundle = PbrBundle {
        mesh: chunk_mesh,
        material: chunk_material.clone(),
        transform: Transform::from_translation(Vec3::zero()),
        ..Default::default()
    };

    commands.spawn(chunk_bundle);

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
