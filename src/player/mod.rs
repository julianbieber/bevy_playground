mod input;
pub mod model;

use crate::movement::model::{Movable, UnitRotation};
use crate::physics::collider::{Collider, ColliderShapes};
use crate::player::input::publish_player_movements;
use crate::player::model::ReceivesInput;
use bevy::prelude::*;

pub struct PlayerMarker;
pub struct PlayerPosition {
    pub position: Vec3,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(player_setup.system())
            .add_system(publish_player_movements.system());
    }
}

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.5 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 1.0, 0.0),
        ..Default::default()
    });
    commands.insert_resource(PlayerPosition {
        position: Vec3::new(0.0, 22.0, 0.0),
    });
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 22.0, 0.0)),
            ..Default::default()
        })
        .insert(Collider {
            collider_shape: ColliderShapes::cube(0.5),
            local_position: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(ReceivesInput)
        .insert(Movable)
        .insert(UnitRotation {
            ..Default::default()
        })
        .insert(PlayerMarker)
        .with_children(|parent| {
            let camera_position = Vec3::new(0.0, 1.0, 5.0);
            let camera_position_y = camera_position.y;
            let up = Vec3::Y;
            let camera_looking_point = -camera_position + 2.0 * camera_position_y * up;
            parent.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_translation(camera_position)
                    .looking_at(camera_looking_point, up),
                ..Default::default()
            });
        });

    let sphere_handle = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 8,
    }));
    let sphere_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 1.0, 0.0),
        ..Default::default()
    });
    commands
        .spawn_bundle(PbrBundle {
            mesh: sphere_handle,
            material: sphere_material_handle,
            transform: Transform::from_translation(Vec3::new(3.0, 22.0, 0.0)),
            ..Default::default()
        })
        .insert(Collider {
            collider_shape: ColliderShapes::Sphere { radius: 0.5 },
            local_position: Vec3::new(0.0, 0.0, 0.0),
        });
}
