mod input;
pub mod model;

use crate::movement::model::{Movable, UnitRotation};
use crate::physics::collider::{Collider, ColliderShapes};
use crate::player::input::publish_player_movements;
use crate::player::model::ReceivesInput;
use bevy::prelude::*;

pub struct PlayerMarker;

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
        albedo: Color::rgb(0.0, 1.0, 0.0),
        ..Default::default()
    });
    commands
        .spawn(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 22.0, 0.0)),
            ..Default::default()
        })
        .with(Collider {
            collider_shape: ColliderShapes::Cuboid {
                half_width_x: 0.25,
                half_height_y: 0.25,
                half_depth_z: 0.25,
            },
            local_position: Vec3::new(0.0, 0.0, 0.0),
        })
        .with(ReceivesInput)
        .with(Movable)
        .with(UnitRotation {
            ..Default::default()
        })
        .with(PlayerMarker)
        .with_children(|parent| {
            let camera_position = Vec3::new(0.0, 1.0, 5.0);
            let camera_position_y = camera_position.y;
            let up = Vec3::unit_y();
            let camera_looking_point = -camera_position + 2.0 * camera_position_y * up;
            parent.spawn(PerspectiveCameraBundle {
                transform: Transform::from_translation(camera_position)
                    .looking_at(camera_looking_point, up),
                ..Default::default()
            });
        });
}
