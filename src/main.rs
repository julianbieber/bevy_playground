mod ai;
mod input;
mod movement;
mod physics;
mod vec3_ext;
mod voxel_world;
mod water;
mod world;

use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::ai::{npc_movement_system, populate_world};
use crate::input::{publish_player_movements, MouseEvents};
use crate::movement::{MoveEvent, MovementReader};
use crate::physics::collider::collision_update;
use movement::{movement_system, player_setup};
use voxel_world::collision::systems::terrain_collision_system;
use water::body_of_water::{
    set_water_position, setup_water_layer, update_material_time, WaterMaterial,
};
use water::water_effect::apply_water_raise;
use world::world_setup;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DiagnosticsPlugin::default())
        .add_asset::<WaterMaterial>()
        .add_startup_system(setup.system())
        .add_startup_system(world_setup.system())
        .add_startup_system(player_setup.system())
        .add_startup_system(populate_world.system())
        .add_startup_system(setup_water_layer.system())
        .add_system(publish_player_movements.system())
        .add_system(npc_movement_system.system())
        .add_system(update_material_time.system())
        .add_system(set_water_position.system())
        .add_system(apply_water_raise.system())
        .add_system(movement_system.system())
        .init_resource::<MouseEvents>()
        .add_event::<MoveEvent>()
        .init_resource::<MovementReader>()
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(terrain_collision_system.system())
        .add_system(collision_update.system())
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
