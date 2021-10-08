use crate::{boundaries::ChunkBoundaries, voxel::VoxelPosition};

pub struct GenericChunk<A, const SIZE_U: usize, const SIZE: i32> {
    pub data: Vec<A>,
    pub boundaries: ChunkBoundaries<SIZE>,
    pub update_age: i32,
}


impl<A, const SIZE_U: usize, const SIZE: i32> GenericChunk<A, SIZE_U, SIZE> {
    fn empty(min: VoxelPosition) -> GenericChunk<A, SIZE_U, SIZE> {
        GenericChunk {
            data: vec![],
            boundaries: ChunkBoundaries::<SIZE>::aligned(min),
            update_age: 0,
        }
    }

    pub fn index(&self, position: VoxelPosition) -> usize {
        let offseted = position - self.boundaries.min;
        (offseted.z * SIZE * SIZE + offseted.y * SIZE + offseted.x) as usize
    }

    pub fn index_to_coord(&self, i: usize) -> VoxelPosition {
        let z = i as i32 / (SIZE * SIZE);
        let y = (i as i32 / SIZE) % SIZE;
        let x = i as i32 % SIZE;
        VoxelPosition { x, y, z } + self.boundaries.min
    }

}

impl<A, const SIZE_U: usize, const SIZE: i32> GenericChunk<A, SIZE_U, SIZE> where A: Copy, A: Clone {
    pub fn get_mut_index(&mut self, i: usize, default: A) -> &mut A {
        if self.data.is_empty() {
            self.data = vec![default; SIZE.pow(3) as usize];
        }
        &mut self.data[i]
    }
    
    pub fn get_index(&self, i: usize, default: A) -> A {
        if self.data.is_empty() {
            default
        } else {
            self.data[i]
        }
    }

    fn reduce_storage(&mut self) {
        self.data = vec![];
    }
}

#[cfg(test)]
mod test {
    use crate::voxel::{Voxel, VoxelPosition};

    use super::GenericChunk;


    #[test]
    fn create_multi_layer_chunks() {
        let meta_meta_chunk = GenericChunk::<GenericChunk<Voxel, 1,1>, 1,1>::empty(VoxelPosition::new(0,0,0), );
    }
}