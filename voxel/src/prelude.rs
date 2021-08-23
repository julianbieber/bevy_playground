use crate::voxel::VoxelPosition;

impl From<[i32; 3]> for VoxelPosition {
    fn from(p: [i32; 3]) -> Self {
        VoxelPosition {
            x: p[0],
            y: p[1],
            z: p[2],
        }
    }
}
