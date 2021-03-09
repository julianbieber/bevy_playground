use crate::voxel_world::voxel::{Voxel, VoxelPosition};

#[derive(Clone)]
pub struct VoxelChunk {
    voxels: Vec<Voxel>,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ChunkBoundaries {
    pub min: [i32; 3],
    pub max: [i32; 3],
}

const CHUNK_SIZE: i32 = 16;

impl ChunkBoundaries {
    pub fn aligned(position: VoxelPosition) -> ChunkBoundaries {
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

    pub fn aligned_boundaries_in(other: &ChunkBoundaries) -> Vec<ChunkBoundaries> {
        let other_length = other.length();
        let min = VoxelPosition {
            x: other.min[0],
            y: other.min[1],
            z: other.min[2],
        };
        let mut boundaries = Vec::with_capacity(
            (other_length[0] / CHUNK_SIZE * other_length[1] / CHUNK_SIZE * other_length[2]
                / CHUNK_SIZE
                + 1) as usize,
        );
        boundaries.push(ChunkBoundaries::aligned(min));
        for x in (min.x..min.x + other_length[0]).step_by(CHUNK_SIZE as usize) {
            for y in (min.y..min.y + other_length[1]).step_by(CHUNK_SIZE as usize) {
                for z in (min.z..min.z + other_length[2]).step_by(CHUNK_SIZE as usize) {
                    boundaries.push(ChunkBoundaries::aligned(VoxelPosition { x, y, z }));
                }
            }
        }

        boundaries
    }

    fn length(&self) -> [i32; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    pub fn contains(&self, position: &VoxelPosition) -> bool {
        position.x >= self.min[0]
            && position.x <= self.max[0]
            && position.y >= self.min[1]
            && position.y <= self.max[1]
            && position.z >= self.min[2]
            && position.z <= self.max[2]
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
