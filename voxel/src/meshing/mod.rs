pub mod consts;
mod terrain_mesh;
mod util;
mod water_mesh;

use bevy::prelude::*;

use crate::DefaultWorldSector;
use terrain_mesh::Meshing;

pub struct VoxelTexture {
    pub material: Handle<StandardMaterial>,
}

pub fn initialize_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    world_sector: Res<DefaultWorldSector>,
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
        material: chunk_material.clone(),
    });

    for mesh in world_sector.initial_terrain_meshes() {
        let chunk_mesh = meshes.add(mesh);
        let chunk_bundle = PbrBundle {
            mesh: chunk_mesh,
            material: chunk_material.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..Default::default()
        };
        commands.spawn_bundle(chunk_bundle);
    }
}