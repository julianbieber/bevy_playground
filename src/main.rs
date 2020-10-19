mod water;
mod world;

use bevy::prelude::*;
use physme::prelude3d::*;

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::render::camera::Camera;
use water::body_of_water::{
    set_water_position, setup_water_layer, update_material_time, WaterMaterial,
};
use water::water_effect::apply_water_raise;
use world::world_setup;

#[derive(Default)]
struct InputState {
    pub reader_motion: EventReader<MouseMotion>,
    pub reader_scroll: EventReader<MouseWheel>,
}
#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

#[derive(Default)]
struct CameraRotation {
    rotation_x: f32,
    rotation_y: f32,
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(Physics3dPlugin)
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
        .init_resource::<State>()
        .init_resource::<CameraRotation>()
        .init_resource::<InputState>()
        .add_system(process_mouse_events.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
fn setup(mut commands: Commands) {
    // Create a new shader pipeline
    commands
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(3.0, 5.0, -8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}
fn process_mouse_events(
    mut state: ResMut<State>,
    mut camera_rotation: ResMut<CameraRotation>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    _camera: &Camera,
    mut transform: Mut<Transform>,
) {
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        let look = event.delta;
        let camera_translation = transform.translation();
        transform.set_translation(Vec3::zero());
        if f32::rem_euclid(
            camera_rotation.rotation_x - (look.y()).to_radians() / 5.0,
            2.0 * std::f32::consts::PI,
        ) > 6.1
        {
            camera_rotation.rotation_x = 6.1
        } else if f32::rem_euclid(
            camera_rotation.rotation_x - (look.y()).to_radians() / 5.0,
            2.0 * std::f32::consts::PI,
        ) < 5.5
        {
            camera_rotation.rotation_x = 5.5
        } else {
            camera_rotation.rotation_x = f32::rem_euclid(
                camera_rotation.rotation_x - (look.y()).to_radians() / 5.0,
                2.0 * std::f32::consts::PI,
            )
        };
        camera_rotation.rotation_y = f32::rem_euclid(
            camera_rotation.rotation_y - (look.x()).to_radians() / 5.0,
            2.0 * std::f32::consts::PI,
        );
        transform.set_rotation(Quat::from_rotation_ypr(
            camera_rotation.rotation_y,
            camera_rotation.rotation_x,
            0.0,
        ));
        transform.set_translation(camera_translation)
    }

    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        let _zoom_delta = event.y;
    }
}
