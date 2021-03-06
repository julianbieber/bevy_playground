use ahash::AHashMap;
use bevy::prelude::Entity;

use super::{chunk::ChunkBoundaries, voxel::VoxelPosition};

pub struct VoxelAccess {
    chunks: AHashMap<ChunkBoundaries, Entity>,
}

impl VoxelAccess {
    pub fn get_chunk_entity_containing(&self, position: VoxelPosition) -> Option<Entity> {
        let boundary = ChunkBoundaries::aligned(position);

        self.chunks.get(&boundary).map(|e| e.clone())
    }

    pub fn add_chunk(&mut self, boundary: ChunkBoundaries, e: Entity) {
        self.chunks.insert(boundary, e);
    }

    pub fn new() -> VoxelAccess {
        VoxelAccess {
            chunks: AHashMap::new(),
        }
    }
}
