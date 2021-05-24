use std::time::Duration;

use bevy::{
    core::{Time, Timer},
    prelude::{Query, Res, ResMut},
};

use crate::voxel_world::{access::VoxelAccess, voxel::VoxelPosition};

use super::water::WaterOperations;

pub struct WaterSource {
    position: VoxelPosition,
    timer: Timer,
}

impl WaterSource {
    pub fn new() -> WaterSource {
        WaterSource {
            position: VoxelPosition { x: 0, y: 40, z: 0 },
            timer: Timer::from_seconds(20.0, true),
        }
    }
}

pub fn water_source(
    mut source_query: Query<(&mut WaterSource,)>,
    _voxel_access: Res<VoxelAccess>,
    mut water_operations: ResMut<WaterOperations>,
    time: Res<Time>,
) {
    for (mut source,) in source_query.iter_mut() {
        if source.timer.tick(time.delta()).finished() {
            source.timer.reset();
            source.timer.set_duration(Duration::from_millis(100));
            water_operations.add(source.position);
        }
    }
}
