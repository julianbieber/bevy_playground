use super::voxel::VoxelPosition;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct ChunkBoundaries<const SIZE: i32> {
    pub min: VoxelPosition,
    pub max: VoxelPosition,
}

pub const CHUNK_SIZE: i32 = 64;

impl<const SIZE: i32> ChunkBoundaries<SIZE> {
    pub fn aligned(position: VoxelPosition) -> ChunkBoundaries<SIZE> {
        let (min_x, max_x) = ChunkBoundaries::<SIZE>::aligned_axis(position.x);
        let (min_y, max_y) = ChunkBoundaries::<SIZE>::aligned_axis(position.y);
        let (min_z, max_z) = ChunkBoundaries::<SIZE>::aligned_axis(position.z);
        ChunkBoundaries {
            min: VoxelPosition::new(min_x, min_y, min_z),
            max: VoxelPosition::new(max_x, max_y, max_z),
        }
    }

    pub fn from_min(min: VoxelPosition) -> ChunkBoundaries<SIZE> {
        ChunkBoundaries {
            min,
            max: min + VoxelPosition::diagonal(SIZE),
        }
    }

    pub fn center(&self) -> VoxelPosition {
        VoxelPosition {
            x: self.min.x + SIZE / 2,
            y: self.min.y + SIZE / 2,
            z: self.min.z + SIZE / 2,
        }
    }

    fn aligned_axis(v: i32) -> (i32, i32) {
        if v >= 0 {
            let m = (v / SIZE) * SIZE;
            (m, m + SIZE)
        } else {
            let m = ((v + 1) / SIZE) * SIZE;
            (m - SIZE, m)
        }
    }

    pub fn in_direction(&self, offset: VoxelPosition) -> ChunkBoundaries<SIZE> {
        ChunkBoundaries {
            min: self.min + (offset * SIZE),
            max: self.max + (offset * SIZE),
        }
    }

    pub fn aligned_boundaries_in(&self) -> Vec<ChunkBoundaries<SIZE>> {
        let other_length = self.length();
        let mut boundaries = Vec::with_capacity(
            (other_length[0] / SIZE * other_length[1] / SIZE * other_length[2] / SIZE + 1) as usize,
        );
        boundaries.push(ChunkBoundaries::aligned(self.min));
        for x in (self.min.x..self.min.x + other_length[0]).step_by(SIZE as usize) {
            for y in (self.min.y..self.min.y + other_length[1]).step_by(SIZE as usize) {
                for z in (self.min.z..self.min.z + other_length[2]).step_by(SIZE as usize) {
                    boundaries.push(ChunkBoundaries::aligned(VoxelPosition {
                        x: x - 1,
                        y: y - 1,
                        z: z - 1,
                    }));
                }
            }
        }

        boundaries
    }

    fn length(&self) -> [i32; 3] {
        [
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        ]
    }

    pub fn contains(&self, position: &VoxelPosition) -> bool {
        position.x >= self.min.x
            && position.x < self.max.x
            && position.y >= self.min.y
            && position.y < self.max.y
            && position.z >= self.min.z
            && position.z < self.max.z
    }
}

#[cfg(test)]
mod test {
    use super::ChunkBoundaries;
    use super::VoxelPosition;
    use super::CHUNK_SIZE;
    #[test]
    fn test_aligned_chunk_boundaries() {
        let aligned_boundaries =
            ChunkBoundaries::<CHUNK_SIZE>::aligned(VoxelPosition { x: 0, y: 0, z: 0 });

        assert_eq!(aligned_boundaries.min, VoxelPosition::diagonal(0));
        assert_eq!(aligned_boundaries.max, VoxelPosition::diagonal(CHUNK_SIZE));

        let aligned_boundaries =
            ChunkBoundaries::<CHUNK_SIZE>::aligned(VoxelPosition::diagonal(CHUNK_SIZE - 1));

        assert_eq!(aligned_boundaries.min, VoxelPosition::diagonal(0));
        assert_eq!(aligned_boundaries.max, VoxelPosition::diagonal(CHUNK_SIZE));

        let aligned_boundaries = ChunkBoundaries::<CHUNK_SIZE>::aligned(VoxelPosition::diagonal(
            CHUNK_SIZE - 1 + CHUNK_SIZE,
        ));
        assert_eq!(aligned_boundaries.min, VoxelPosition::diagonal(CHUNK_SIZE));
        assert_eq!(
            aligned_boundaries.max,
            VoxelPosition::diagonal(2 * CHUNK_SIZE)
        );

        let aligned_boundaries = ChunkBoundaries::<CHUNK_SIZE>::aligned(VoxelPosition {
            x: -1,
            y: -1,
            z: -1,
        });
        assert_eq!(aligned_boundaries.min, VoxelPosition::diagonal(-CHUNK_SIZE));
        assert_eq!(aligned_boundaries.max, VoxelPosition::diagonal(0));
    }

    #[test]
    fn test_aligned_chunk_boundaries_at_max() {
        let aligned_boundaries =
            ChunkBoundaries::<CHUNK_SIZE>::aligned(VoxelPosition::diagonal(-64));

        assert_eq!(aligned_boundaries.min, VoxelPosition::diagonal(-64));
    }

    #[test]
    fn boundary_over_position_and_contains_consistency() {
        let position = VoxelPosition {
            x: -45,
            y: -120,
            z: -93,
        };

        let alingned = ChunkBoundaries::aligned(position);

        let matching_boundary =
            ChunkBoundaries::<CHUNK_SIZE>::from_min(VoxelPosition::new(-64, -128, -128));

        assert_eq!(alingned, matching_boundary);
    }
}
