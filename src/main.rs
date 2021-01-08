mod ai;
mod delayed_despawn;
mod movement;
mod particles;
mod physics;
mod player;
mod vec3_ext;
mod voxel_world;
mod water;
mod world;

use bevy::prelude::*;

use crate::ai::AIPlugin;
use crate::delayed_despawn::DelayedDespawnsPlugin;
use crate::movement::MovementPlugin;
use crate::particles::ParticlePlugin;
use crate::physics::collider::collision_update;
use crate::player::PlayerPlugin;
use crate::water::WaterPlugin;
use voxel_world::collision::systems::terrain_collision_system;
use world::world_setup;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlePlugin)
        .add_plugin(WaterPlugin)
        .add_plugin(AIPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DelayedDespawnsPlugin)
        .add_startup_system(window_setup.system())
        .add_startup_system(world_setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(terrain_collision_system.system())
        .add_system(collision_update.system())
        .run();
}

fn window_setup(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_lock_mode(true);
    windows
        .get_primary_mut()
        .unwrap()
        .set_cursor_visibility(false);
}
