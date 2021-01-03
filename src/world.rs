use bevy::prelude::*;

use crate::physics::collider::{Collider, ColliderShapes};
use crate::voxel_world::generator::VoxelWorld;

pub fn world_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let w = VoxelWorld::generate(150, 150, rand::thread_rng());
    w.add_to_world(commands, asset_server, &mut meshes, &mut materials);
    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 100.0, 4.0)),
        ..Default::default()
    });

    let cube_handle = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 8,
    }));
    let cube_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.0, 1.0, 0.0),
        ..Default::default()
    });
    commands
        // parent cube
        .spawn(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 45.0)),
            ..Default::default()
        })
        .with(Collider {
            collider_shape: ColliderShapes::cube(0.5),
            local_position: Vec3::new(0.0, 0.0, 0.0),
        });
}
