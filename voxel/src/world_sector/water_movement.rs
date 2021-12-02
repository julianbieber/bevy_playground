use std::cell::{RefCell, RefMut};

use crate::voxel::Voxel;

use super::{grid::GridWorld, pillar::VoxelPillar};

pub fn update_water(world_sector: &mut GridWorld) {
    //world_sector.iterate_mut(update_single_water_block);
}

fn update_single_water_block(
    mut center: RefMut<VoxelPillar>,
    mut left: Option<RefMut<VoxelPillar>>,
    mut up: Option<RefMut<VoxelPillar>>,
    mut right: Option<RefMut<VoxelPillar>>,
    mut down: Option<RefMut<VoxelPillar>>,
) {
    center.voxel_heights[0].lower_voxel = Voxel::Nothing;
    for center_index in 1..center.voxel_heights.len() {
        let lower_index = center_index - 1;

        let lower_type = center.voxel_heights[lower_index].upper_voxel;
    }
}
