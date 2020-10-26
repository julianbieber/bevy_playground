mod water;
mod world;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use physme::prelude3d::*;

use bevy::input::mouse::MouseMotion;
use water::body_of_water::{
    set_water_position, setup_water_layer, update_material_time, WaterMaterial,
};
use water::water_effect::apply_water_raise;
use world::world_setup;

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

#[derive(Default)]
struct CameraRotation {
    rotation_x: f32,
    rotation_y: f32,
}

/// this component indicates what entities should rotate
#[derive(Default)]
struct Rotator;

/// rotates the parent, which will result in the child also rotating
fn rotator_system(
    mut state: ResMut<State>,
    mut camera_rotation: ResMut<CameraRotation>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Rotator, &mut Transform)>,
) {
    for (_rotator, mut transform) in &mut query.iter() {
        for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
            let look = event.delta;
            let camera_translation = transform.translation;
            transform.translation = Vec3::zero();
            if (f32::rem_euclid(
                camera_rotation.rotation_y - (look.y()).to_radians() / 5.0,
                2.0 * std::f32::consts::PI,
            ) - std::f32::consts::FRAC_PI_2)
                .abs()
                < 0.2
            {
            } else if (f32::rem_euclid(
                camera_rotation.rotation_y - (look.y()).to_radians() / 5.0,
                2.0 * std::f32::consts::PI,
            ) - 3.0 * std::f32::consts::FRAC_PI_2)
                .abs()
                < 0.2
            {
            } else {
                camera_rotation.rotation_y = f32::rem_euclid(
                    camera_rotation.rotation_y - (look.y()).to_radians() / 5.0,
                    2.0 * std::f32::consts::PI,
                );
            };
            camera_rotation.rotation_x = f32::rem_euclid(
                camera_rotation.rotation_x - (look.x()).to_radians() / 5.0,
                2.0 * std::f32::consts::PI,
            );
            transform.rotation = Quat::from_rotation_ypr(
                camera_rotation.rotation_x,
                camera_rotation.rotation_y,
                0.0,
            );
            transform.translation = camera_translation;
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
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(Physics3dPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_resource(GlobalGravity(Vec3::new(0.0, -9.8, 0.0)))
        .add_resource(GlobalFriction(0.90))
        .add_resource(GlobalStep(0.5))
        .add_asset::<WaterMaterial>()
        .add_startup_system(setup.system())
        .add_startup_system(world_setup.system())
        .add_startup_system(setup_water_layer.system())
        .add_system(update_material_time.system())
        .add_system(set_water_position.system())
        .add_system(apply_water_raise.system())
        .add_system(rotator_system.system())
        .init_resource::<State>()
        .init_resource::<CameraRotation>()
        .init_resource::<Rotator>()
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_lock_mode(true);
    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_visibility(false);

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
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(Rotator)
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
