use crate::{boundaries::ChunkBoundaries, voxel::{Voxel, VoxelDirection, VoxelPosition}};

pub struct MetaChunk<A, const SIZE_U: usize, const SIZE: i32, const SQUARE: usize> {
    pub data: Vec<A>,
    pub min: VoxelPosition,
}


impl<A, const SIZE_U: usize, const SIZE: i32, const SQUARE: usize> MetaChunk<A, SIZE_U, SIZE, SQUARE> {

    /// gets the neighboring index, if the index is out of bounds, the second return value is false.
    /// In this case the calculated index assumes, there is another chunk of the same size next to the current chunk.
    pub fn get_neighbor_index(&self, i: usize, direction: VoxelDirection) -> (usize, bool) {
        let cubed = SIZE_U.pow(3);
        match direction {
            VoxelDirection::UP => {
                if i % SQUARE >= (SQUARE - SIZE_U) {
                    (i - (SQUARE - SIZE_U), false)
                } else {
                    (i + SIZE_U, true) 
                }
            }
            VoxelDirection::DOWN => {
                if i % SQUARE < SIZE_U {
                    (i + (SQUARE - SIZE_U), false)
                } else {
                    (i - SIZE_U, true)
                }
            }
            VoxelDirection::LEFT => {
                if i % SIZE_U == 0 {
                    (i + (SIZE_U - 1), false)
                } else {
                    (i - 1, true)
                }
            }
            VoxelDirection::RIGHT => {
                if i % SIZE_U == SIZE_U - 1 {
                    (i - (SIZE_U - 1), false)
                } else {
                    (i + 1, true)
                }
            }
            VoxelDirection::FRONT => {
                if i < SQUARE {
                    (i + cubed - SQUARE, false)
                } else {
                    (i - SQUARE, true)
                }
            }
            VoxelDirection::BACK => {
                if i >= (cubed - SQUARE) {
                    (i - (cubed - SQUARE), false)
                } else {
                    (i + SQUARE, true)
                }
            }
        }
    }

    pub fn surrounding<const I: usize>(
        &self,
        i: usize,
        directions: [VoxelDirection; I],
    ) -> [(VoxelDirection, (usize, bool)); I] {
        directions.map(|d| {
            let index = self.get_neighbor_index(i, d);
            (d, index)
        })
    }
    

    pub fn indices_at_boundary(
        &self,
        direction: VoxelDirection,
    ) -> [usize; SQUARE] {
        let mut values = [0; SQUARE];
        match direction {
            VoxelDirection::UP => {
                for i in 0..SQUARE {
                    values[i] = SQUARE - SIZE_U + i % SIZE_U + (i / SIZE_U * SQUARE)
                }
            }
            VoxelDirection::DOWN => {
                for i in 0..SQUARE {
                    values[i] = i % SIZE_U + (i / SIZE_U) * SQUARE
                }
            }
            VoxelDirection::LEFT => {
                for i in 0..SQUARE {
                    values[i] = i * SIZE_U;
                }
            }
            VoxelDirection::RIGHT => {
                for i in 0..SQUARE {
                    values[i] = i * SIZE_U + SIZE_U - 1;
                }
            }
            VoxelDirection::FRONT => {
                for i in 0..SQUARE {
                    values[i] = i
                }
            }
            VoxelDirection::BACK => {
                for i in 0..SQUARE {
                    values[i] = i + SIZE_U.pow(3) - SQUARE
                }
            }
        }
        values
    }
}

pub struct VoxelChunk<> {
    
}

/// This impl is used for chunks of copyable data, that can be replaced with a default representation to save memory.
/// For example for singular voxels.
/// If the type boundary is fulfilled, data should not be accessed directly.
impl<const SIZE_U: usize, const SIZE: i32, const SQUARE: usize> MetaChunk<Voxel, (Voxel, i32), SIZE_U, SIZE, SQUARE> {
    pub fn empty(min: VoxelPosition, default_value: Voxel) -> MetaChunk<Voxel, (Voxel, i32), SIZE_U, SIZE, SQUARE>{
        MetaChunk {
            data: vec![],
            min: ChunkBoundaries::<SIZE>::aligned(min).min,
            meta_data: (default_value, 0)
        }
    }

    pub fn get_mut_index(&mut self, i: usize) -> &mut Voxel {
        if self.data.is_empty() {
            self.data = vec![self.meta_data.0; SIZE.pow(3) as usize];
        }
        &mut self.data[i]
    }
    
    pub fn get_index(&self, i: usize) -> &Voxel {
        if self.data.is_empty() {
            &self.meta_data.0
        } else {
            &self.data[i]
        }
    }

    pub fn reduce_storage(&mut self, new_default: Voxel) {
        self.data = vec![];
        self.meta_data.0 = new_default;
    }

    pub fn count_outside_land_voxels(&self, direction: VoxelDirection) -> usize {
        if self.data.len() == 0 {
            return match self.meta_data.0 {
                Voxel::LandVoxel { typ } => SIZE_U.pow(3),
                Voxel::WaterVoxel { fill } => 0,
                Voxel::Nothing => 0,
            }
        }

        let mut c = 0;
        for i in self.indices_at_boundary(direction) {
            match self.data[i] {
                Voxel::LandVoxel { typ } => c += 1,
                Voxel::WaterVoxel { fill } => (),
                Voxel::Nothing => (),
            }
        }
        c
    }

    pub fn local_index(&self, position: VoxelPosition) -> usize {
        let offseted = position - self.min;
        (offseted.z * SIZE * SIZE + offseted.y * SIZE + offseted.x) as usize
    }

    pub fn index_to_coord(&self, i: usize) -> VoxelPosition {
        let z = i as i32 / (SIZE * SIZE);
        let y = (i as i32 / SIZE) % SIZE;
        let x = i as i32 % SIZE;
        VoxelPosition { x, y, z } + self.min
    }
}

#[cfg(test)]
mod test {
    use crate::{boundaries::ChunkBoundaries, voxel::{VoxelDirection, VoxelPosition}};

    use super::MetaChunk;


    #[test]
    fn walt_through_chunk() {
        let chunk = MetaChunk::<i32, i32, 8, 8, 64> {
            data: vec![1; 8],
            min: ChunkBoundaries::<8>::aligned(VoxelPosition::new(0,0,0)).min,
            meta_data: 0
        };

        let mut i = 0;
        for direction in [VoxelDirection::RIGHT, VoxelDirection::UP, VoxelDirection::BACK, VoxelDirection::DOWN, VoxelDirection::LEFT, VoxelDirection::FRONT] {
            for _ in 0..7 {
                let (new_i, same_chunk) = chunk.get_neighbor_index(i, direction);
                dbg!(i, direction, new_i, same_chunk);
                assert!(same_chunk);
                i = new_i;
            }
        }
        assert_eq!(i, 0);
    }
}