use std::{ops::AddAssign, time::Duration};

use bevy::{
    core::{Time, Timer},
    prelude::{Query, Res},
};

use crate::{access::VoxelAccess, voxel::VoxelPosition};

use super::water::Water;

pub struct WaterSource {
    position: VoxelPosition,
    timer: Timer,
}

impl WaterSource {
    pub fn new(position: VoxelPosition) -> WaterSource {
        WaterSource {
            position,
            timer: Timer::from_seconds(20.0, true),
        }
    }
}

pub fn water_source(
    mut source_query: Query<(&mut WaterSource,)>,
    mut water_query: Query<(&mut Water,)>,
    _voxel_access: Res<VoxelAccess>,
    time: Res<Time>,
) {
    for (mut source,) in source_query.iter_mut() {
        if source.timer.tick(time.delta()).finished() {
            source.timer.reset();
            source.timer.set_duration(Duration::from_millis(100));
            for (mut water,) in water_query.iter_mut() {
                water
                    .changed
                    .entry(source.position)
                    .or_insert(0.0)
                    .add_assign(0.5);
            }
        }
    }
}
