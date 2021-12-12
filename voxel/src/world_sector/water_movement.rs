use std::cell::{RefCell, RefMut};

use crate::voxel::Voxel;

use super::{grid::GridWorld, pillar::VoxelPillar};

pub fn update_water(world_sector: &mut GridWorld) {
    world_sector.iterate_mut(1, 1, update_single_water_block);
}

fn update_single_water_block(
    center: &mut VoxelPillar,
    left: Option<&mut VoxelPillar>,
    up: Option<&mut VoxelPillar>,
    right: Option<&mut VoxelPillar>,
    down: Option<&mut VoxelPillar>,
) {
    for center_index in 1..center.voxel_heights.len() {
        let lower_index = center_index - 1;

        let lower_type = center.voxel_heights[lower_index].upper_voxel;
        let my_type = center.voxel_heights[center_index].lower_voxel;

        


    }
}
