mod water;
mod world;

use bevy::prelude::*;
use physme::prelude3d::*;

use water::{apply_water_raise, setup_water_layer, update_material_time, WaterMaterial};
use world::world_setup;

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
        .add_system(apply_water_raise.system())
        .run();
}
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
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
