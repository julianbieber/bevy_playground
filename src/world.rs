use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::voxel_world::generator::VoxelWorld;
use crate::{
    physics::collider::{Collider, ColliderShapes},
    voxel_world::world_structure::Terrain,
};
use flume::{unbounded, Receiver, Sender};
use rand::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (tx, rx) = unbounded::<(Mesh, Terrain, Option<Entity>)>();
        app.insert_resource(tx)
            .insert_resource(rx)
            .add_system(update_world_from_channel.system());
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
