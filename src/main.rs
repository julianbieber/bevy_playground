mod ai;
mod input;
mod movement;
mod particles;
mod physics;
mod vec3_ext;
mod voxel_world;
mod water;
mod world;

use bevy::prelude::*;

use crate::ai::movement::{npc_movement_system, update_behaviour_system};
use crate::ai::spawn::{enemy_spawn_system, SpawnCoolDown};
use crate::input::{publish_player_movements, MouseEvents};
use crate::movement::{MoveEvent, MovementReader};
use crate::particles::ParticlePlugin;
use crate::physics::collider::collision_update;
use crate::water::WaterPlugin;
use movement::{movement_system, player_setup};
use voxel_world::collision::systems::terrain_collision_system;
use world::world_setup;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ParticlePlugin)
        .add_plugin(WaterPlugin)
        .add_resource(SpawnCoolDown {
            timer: Timer::from_seconds(2.0, true),
        })
        .add_startup_system(setup.system())
        .add_startup_system(world_setup.system())
        .add_startup_system(player_setup.system())
        .add_system(publish_player_movements.system())
        .add_system(npc_movement_system.system())
        .add_system(update_behaviour_system.system())
        .add_system(movement_system.system())
        .add_system(enemy_spawn_system.system())
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
