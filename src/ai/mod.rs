mod model;
mod movement;
mod spawn;

use crate::ai::movement::{npc_movement_system, update_behaviour_system};
use crate::ai::spawn::{enemy_spawn_system, SpawnCoolDown};
use bevy::prelude::*;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(SpawnCoolDown {
            timer: Timer::from_seconds(2.0, true),
        })
        .add_system(npc_movement_system.system())
        .add_system(update_behaviour_system.system())
        .add_system(enemy_spawn_system.system());
    }
}
