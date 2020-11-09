use crate::water::water_effect::WaterEffected;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use physme::prelude3d::*;

#[derive(Default)]
pub struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

#[derive(Default)]
pub struct PlayerRotation {
    rotation_x: f32,
    rotation_y: f32,
}

/// this component indicates what entities should rotate
#[derive(Default)]
pub struct Rotator;

/// rotates the parent, which will result in the child also rotating
pub fn rotator_system(
    mut state: ResMut<State>,
    mut player_rotation: ResMut<PlayerRotation>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Rotator, &mut Transform, &mut RigidBody)>,
) {
    for (_rotator, mut transform, mut rigibody) in query.iter_mut() {
        for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
            let look = event.delta;
            let player_translation = transform.translation;
            transform.translation = Vec3::zero();
            if (f32::rem_euclid(
                player_rotation.rotation_y - (look.y()).to_radians() / 5.0,
                2.0 * std::f32::consts::PI,
            ) - std::f32::consts::FRAC_PI_2)
                .abs()
                < 0.2
            {
            } else if (f32::rem_euclid(
                player_rotation.rotation_y - (look.y()).to_radians() / 5.0,
                2.0 * std::f32::consts::PI,
            ) - 3.0 * std::f32::consts::FRAC_PI_2)
                .abs()
                < 0.2
            {
            } else {
                player_rotation.rotation_y = f32::rem_euclid(
                    player_rotation.rotation_y - (look.y()).to_radians() / 5.0,
                    2.0 * std::f32::consts::PI,
                );
            };
            player_rotation.rotation_x = f32::rem_euclid(
                player_rotation.rotation_x - (look.x()).to_radians() / 5.0,
                2.0 * std::f32::consts::PI,
            );
            transform.rotation = Quat::from_rotation_ypr(
                player_rotation.rotation_x,
                player_rotation.rotation_y,
                0.0,
            );
            transform.translation = player_translation;
        }
        if keys.pressed(KeyCode::W) {
            let a = transform.rotation.mul_vec3(Vec3::new(0.0, 0.0, -0.1));
            transform.translation += a;
        }
        if keys.pressed(KeyCode::A) {
            let a = transform.rotation.mul_vec3(Vec3::new(-0.1, 0.0, 0.0));
            transform.translation += a;
        }
        if keys.pressed(KeyCode::D) {
            let a = transform.rotation.mul_vec3(Vec3::new(0.1, 0.0, 0.0));
            transform.translation += a;
        }
        if keys.pressed(KeyCode::S) {
            let a = transform.rotation.mul_vec3(Vec3::new(0.0, 0.0, 0.1));
            transform.translation += a;
        }
        if keys.pressed(KeyCode::Q) {
            let a = transform.rotation.mul_vec3(Vec3::new(0.0, 0.1, 0.0));
            transform.translation += a;
        }
        if keys.pressed(KeyCode::E) {
            let a = transform.rotation.mul_vec3(Vec3::new(0.0, -0.1, 0.0));
            transform.translation += a;
        }
        rigibody.position = transform.translation;
        rigibody.rotation = transform.rotation;
    }
}

pub fn camera_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.5 }));
    let cube_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });
    commands
        // parent cube
        .spawn(PbrComponents {
            mesh: cube_handle.clone(),
            material: cube_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Real(1.0))
                .with_status(Status::Semikinematic)
                .with_position(Vec3::new(0.0, 5.0, 0.0)),
        )
        .with(Rotator)
        .with(WaterEffected::new())
        .with_children(|parent| {
            parent.spawn((Shape::from(Size3::new(1.0, 1.0, 1.0)),));
        })
        .with_children(|parent| {
            let camera_position = Vec3::new(0.0, 1.0, 5.0);
            let camera_position_y = camera_position.y();
            let up = Vec3::unit_y();
            let camera_looking_point = -camera_position + 2.0 * camera_position_y * up;
            parent.spawn(Camera3dComponents {
                transform: Transform::from_translation(camera_position)
                    .looking_at(camera_looking_point, up),
                ..Default::default()
            });
        });
}