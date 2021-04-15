mod effects;
mod evaluation;
mod internal_model;
pub mod model;
mod world_gen;

use ahash::AHashMap;
use bevy::prelude::*;

use crate::voxel_world::{boundaries::ChunkBoundaries, generator::VoxelWorld};
use crate::{
    physics::collider::{Collider, ColliderShapes},
    voxel_world::chunk::VoxelChunk,
};
use flume::unbounded;
use rand::prelude::*;

use self::{
    effects::{erosion, move_floating_voxels},
    evaluation::{
        evaluate_delayed_transformations, update_world_event_reader, update_world_from_channel,
    },
    model::{DelayedWorldTransformations, WorldUpdateEvent, WorldUpdateResult},
    world_gen::{read_generation_results, setup_world_gen, start_generation},
};

pub struct VoxelTexture {
    pub material: Handle<StandardMaterial>,
}

pub struct AdditionalVoxels {
    voxels: AHashMap<ChunkBoundaries, VoxelChunk>,
}
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
            .add_startup_system(world_setup.system())
            .add_startup_system(setup_world_gen.system())
            .add_system(start_generation.system())
            .add_system(read_generation_results.system());
    }
}

fn world_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let chunk_texture = asset_server.load("world_texture_color.png");
    let chunk_roughness = asset_server.load("world_texture_roughnes.png");
    let chunk_normal = asset_server.load("world_texture_normal.png");

    let chunk_material = materials.add(StandardMaterial {
        base_color_texture: Some(chunk_texture),
        metallic_roughness_texture: Some(chunk_roughness),
        metallic: 0.2,
        roughness: 1.0,
        normal_map: Some(chunk_normal),
        ..Default::default()
    });
    commands.insert_resource(VoxelTexture {
        material: chunk_material,
    });

    let w = VoxelWorld::generate(150, 150, SmallRng::from_entropy());
    let mut chunk_map = AHashMap::new();
    for pillar in w.pillars {
        for voxel in pillar.voxels() {
            let matching_boundary = ChunkBoundaries::aligned(voxel.position);
            let m = matching_boundary.clone();
            chunk_map
                .entry(matching_boundary)
                .or_insert(VoxelChunk::empty(m))
                .set(voxel);
        }
    }
    commands.insert_resource(AdditionalVoxels { voxels: chunk_map });

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
