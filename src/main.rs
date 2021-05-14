mod ai;
mod delayed_despawn;
mod movement;
mod particles;
mod physics;
mod pickups;
mod player;
mod unit_effects;
mod voxel_world;
mod world;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use pickups::EnergyPlugin;
use unit_effects::DelayedUnitEffectsPlugin;

use crate::ai::AIPlugin;
use crate::delayed_despawn::DelayedDespawnsPlugin;
use crate::movement::MovementPlugin;
use crate::particles::ParticlePlugin;
use crate::physics::collider::collision_update;
use crate::player::PlayerPlugin;
use crate::voxel_world::water::WaterPlugin;
use voxel_world::{access::VoxelAccess, collision::systems::terrain_collision_system};
use world::WorldPlugin;
#[macro_use]
extern crate itertools;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    App::build()
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(8))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldPlugin)
        .add_plugin(ParticlePlugin)
        .add_plugin(WaterPlugin)
        .add_plugin(AIPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DelayedDespawnsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EnergyPlugin)
        .add_plugin(DelayedUnitEffectsPlugin)
        // Adds a system that prints diagnostics to the console
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(window_setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .insert_resource(VoxelAccess::new())
        .add_system(terrain_collision_system.system())
        .add_system(collision_update.system())
        .run();
}

fn window_setup(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
    window.set_maximized(true);
    window.set_vsync(false);
}
