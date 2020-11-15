use bevy::prelude::*;

use crate::world_generation::voxel_world::VoxelWorld;

pub fn world_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let w = VoxelWorld::generate(50, 50, rand::thread_rng());
    w.add_to_world(commands, asset_server, meshes, materials);
    commands.spawn(LightComponents {
        transform: Transform::from_translation(Vec3::new(4.0, 100.0, 4.0)),
        ..Default::default()
    });
}
