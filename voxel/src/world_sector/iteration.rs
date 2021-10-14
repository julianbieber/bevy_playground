use crate::voxel::{Voxel, VoxelDirection};

use super::VoxelChunk;


pub trait IterateOverLocalBlocks<ChunkMetaData> {
    fn iter_mut<const I: usize>(&mut self, f: fn([(VoxelDirection, Option<&mut Voxel>); I]), predicate: fn(&ChunkMetaData) -> bool, meta_update: fn(&mut ChunkMetaData));
}

impl IterateOverLocalBlocks<> for VoxelChunk {
    
}