use ahash::AHashMap;

use crate::voxel_world::voxel::{Voxel, VoxelPosition};

#[derive(Clone)]
pub struct VoxelChunk {
    voxels: AHashMap<i32, AHashMap<i32, AHashMap<i32, Voxel>>>,
    count: usize,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ChunkBoundaries {
    pub min: [i32; 3],
    pub max: [i32; 3],
}

const CHUNK_SIZE: i32 = 32;

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
        VoxelChunk {
            voxels: AHashMap::new(),
            count: 0,
        }
    }

    pub fn get_voxels(&self) -> Vec<Voxel> {
        let mut voxels = Vec::with_capacity(self.count);
        for (_, xs) in self.voxels.iter() {
            for (_, ys) in xs.iter() {
                for (_, voxel) in ys.iter() {
                    voxels.push(voxel.clone());
                }
            }
        }

        voxels
    }

    pub fn set(&mut self, voxel: Voxel) {
        if let None = self
            .voxels
            .entry(voxel.position.x)
            .or_insert(AHashMap::new())
            .entry(voxel.position.y)
            .or_insert(AHashMap::new())
            .insert(voxel.position.z, voxel)
        {
            self.count += 1;
        }
    }

    pub fn remove(&mut self, position: VoxelPosition) -> Option<Voxel> {
        let old = self
            .voxels
            .entry(position.x)
            .or_insert(AHashMap::new())
            .entry(position.y)
            .or_insert(AHashMap::new())
            .remove(&position.z);
        if old.is_some() {
            self.count -= 1;
        }
        old
    }

    pub fn get(&self, position: &VoxelPosition) -> Option<&Voxel> {
        self.voxels
            .get(&position.x)
            .and_then(|ys| ys.get(&position.y))
            .and_then(|zs| zs.get(&position.z))
    }
}
