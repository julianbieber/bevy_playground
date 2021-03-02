use crate::voxel_world::voxel::{Voxel, VoxelPosition};
use bevy::prelude::Vec3;

pub struct VoxelChunk {
    voxels: Vec<Voxel>,
}

#[derive(Eq, PartialEq, Hash)]
pub struct ChunkBoundaries {
    min: [i32; 3],
    max: [i32; 3],
}

const CHUNK_SIZE: i32 = 16;

impl ChunkBoundaries {
    fn aligned(position: VoxelPosition) -> ChunkBoundaries {
        let min = [
            position.x / CHUNK_SIZE,
            position.y / CHUNK_SIZE,
            position.z / CHUNK_SIZE,
        ];
        ChunkBoundaries {
            min,
            max: [
                min[0] + CHUNK_SIZE - 1,
                min[1] + CHUNK_SIZE - 1,
                min[2] + CHUNK_SIZE - 1,
            ],
        }
    }
}

impl VoxelChunk {
    pub fn empty() -> VoxelChunk {
        VoxelChunk { voxels: Vec::new() }
    }

    pub fn boundaries(&self) -> ChunkBoundaries {
        let mut min = [i32::max_value(), i32::max_value(), i32::max_value()];
        let mut max = [i32::min_value(), i32::min_value(), i32::min_value()];

        for voxel in self.voxels.iter() {
            if voxel.position.x < min[0] {
                min[0] = voxel.position.x;
            }
            if voxel.position.y < min[1] {
                min[1] = voxel.position.y;
            }
            if voxel.position.z < min[2] {
                min[2] = voxel.position.z;
            }

            if voxel.position.x > max[0] {
                max[0] = voxel.position.x;
            }
            if voxel.position.y > max[1] {
                max[1] = voxel.position.y;
            }
            if voxel.position.z > max[2] {
                max[2] = voxel.position.z;
            }
        }

        ChunkBoundaries { min, max }
    }

    pub fn get_voxels(&self) -> &Vec<Voxel> {
        &self.voxels
    }

    pub fn set(&mut self, voxel: Voxel) {
        if let Some(old) = self
            .voxels
            .iter_mut()
            .find(|v| v.position == voxel.position)
        {
            *old = voxel;
        } else {
            self.voxels.push(voxel);
        }
    }

    pub fn remove(&mut self, position: VoxelPosition) -> Option<Voxel> {
        let vec_position = self
            .voxels
            .iter()
            .enumerate()
            .find(|(_, v)| v.position == position)
            .map(|t| t.0);
        vec_position.map(|i| self.voxels.remove(i))
    }

    pub fn get(&self, position: &VoxelPosition) -> Option<&Voxel> {
        self.voxels.iter().find(|v| v.position == *position)
    }
}
