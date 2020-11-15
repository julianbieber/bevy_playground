mod camera;
mod physics;
mod render;
mod water;
mod world;
mod world_generation;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;

use camera::{camera_setup, rotator_system, PlayerRotation, Rotator, State};
use render::{hide_far_away, update_player_position};
use water::body_of_water::{
    set_water_position, setup_water_layer, update_material_time, WaterMaterial,
};
use water::water_effect::apply_water_raise;
use world::world_setup;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_asset::<WaterMaterial>()
        .add_startup_system(setup.system())
        .add_startup_system(world_setup.system())
        .add_startup_system(camera_setup.system())
        .add_startup_system(setup_water_layer.system())
        .add_system(update_material_time.system())
        .add_system(set_water_position.system())
        .add_system(apply_water_raise.system())
        .add_system(rotator_system.system())
        .init_resource::<State>()
        .init_resource::<PlayerRotation>()
        .init_resource::<Rotator>()
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(hide_far_away.system())
        .add_system(update_player_position.system())
        .run();
}
fn setup(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_lock_mode(true);
    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_visibility(false);
}
