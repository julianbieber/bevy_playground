use crate::voxel::Voxel;

#[derive(Default, Clone)]
pub struct VoxelPillar {
    pub voxel_heights: Vec<VoxelDescription>,
}

#[derive(Clone)]
pub struct VoxelDescription {
    pub min: i32,
    pub max: i32,
    pub upper_voxel: Voxel,
    pub lower_voxel: Voxel,
}
