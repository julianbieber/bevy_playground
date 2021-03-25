use super::voxel::VoxelPosition;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct ChunkBoundaries {
    pub min: [i32; 3],
    pub max: [i32; 3],
}

const CHUNK_SIZE: i32 = 64;

impl ChunkBoundaries {
    pub fn aligned(position: VoxelPosition) -> ChunkBoundaries {
        let (min_x, max_x) = ChunkBoundaries::aligned_axis(position.x);
        let (min_y, max_y) = ChunkBoundaries::aligned_axis(position.y);
        let (min_z, max_z) = ChunkBoundaries::aligned_axis(position.z);
        ChunkBoundaries {
            min: [min_x, min_y, min_z],
            max: [max_x, max_y, max_z],
        }
    }

    pub fn center(&self) -> VoxelPosition {
        VoxelPosition {
            x: self.min[0] + CHUNK_SIZE / 2,
            y: self.min[0] + CHUNK_SIZE / 2,
            z: self.min[0] + CHUNK_SIZE / 2,
        }
    }

    fn aligned_axis(v: i32) -> (i32, i32) {
        if v >= 0 {
            let m = (v / CHUNK_SIZE) * CHUNK_SIZE;
            (m, m + CHUNK_SIZE)
        } else {
            let m = (v / CHUNK_SIZE) * CHUNK_SIZE;
            (m - CHUNK_SIZE, m)
        }
    }

    pub fn in_direction(&self, offset: [i32; 3]) -> ChunkBoundaries {
        ChunkBoundaries {
            min: [
                self.min[0] + (offset[0] * CHUNK_SIZE),
                self.min[1] + (offset[1] * CHUNK_SIZE),
                self.min[2] + (offset[2] * CHUNK_SIZE),
            ],
            max: [
                self.max[0] + (offset[0] * CHUNK_SIZE),
                self.max[1] + (offset[1] * CHUNK_SIZE),
                self.max[2] + (offset[2] * CHUNK_SIZE),
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

#[cfg(test)]
mod test {
    use super::ChunkBoundaries;
    use super::VoxelPosition;
    use super::CHUNK_SIZE;
    #[test]
    fn test_aligned_chunk_boundaries() {
        let aligned_boundaries = ChunkBoundaries::aligned(VoxelPosition { x: 0, y: 0, z: 0 });

        assert_eq!(aligned_boundaries.min, [0, 0, 0]);
        assert_eq!(aligned_boundaries.max, [CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE]);

        let aligned_boundaries = ChunkBoundaries::aligned(VoxelPosition {
            x: CHUNK_SIZE - 1,
            y: CHUNK_SIZE - 1,
            z: CHUNK_SIZE - 1,
        });
        assert_eq!(aligned_boundaries.min, [0, 0, 0]);
        assert_eq!(aligned_boundaries.max, [CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE]);

        let aligned_boundaries = ChunkBoundaries::aligned(VoxelPosition {
            x: CHUNK_SIZE - 1 + CHUNK_SIZE,
            y: CHUNK_SIZE - 1 + CHUNK_SIZE,
            z: CHUNK_SIZE - 1 + CHUNK_SIZE,
        });
        assert_eq!(aligned_boundaries.min, [CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE]);
        assert_eq!(
            aligned_boundaries.max,
            [2 * CHUNK_SIZE, 2 * CHUNK_SIZE, 2 * CHUNK_SIZE]
        );

        let aligned_boundaries = ChunkBoundaries::aligned(VoxelPosition {
            x: -1,
            y: -1,
            z: -1,
        });
        assert_eq!(
            aligned_boundaries.min,
            [-CHUNK_SIZE, -CHUNK_SIZE, -CHUNK_SIZE]
        );
        assert_eq!(aligned_boundaries.max, [0, 0, 0]);
    }
}
