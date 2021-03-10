use ahash::AHashMap;
use bevy::prelude::{Entity, Query};

use super::{
    chunk::{ChunkBoundaries, VoxelChunk},
    voxel::VoxelPosition,
};

pub struct VoxelAccess {
    chunks: AHashMap<ChunkBoundaries, Entity>,
}

impl VoxelAccess {
    pub fn get_chunk_entity_containing(&self, position: VoxelPosition) -> Option<Entity> {
        let boundary = ChunkBoundaries::aligned(position);

        self.chunks.get(&boundary).map(|e| e.clone())
    }

    pub fn get_chunk_entity(&self, boundary: &ChunkBoundaries) -> Option<Entity> {
        self.chunks.get(boundary).map(|e| e.clone())
    }

    pub fn add_chunk(&mut self, boundary: ChunkBoundaries, e: Entity) {
        self.chunks.insert(boundary, e);
    }

    /// query is only mutable since immutable .get calls are not implemented if the inner type is mutable; the inner type is mutable since the caller needs to mutaute the chunk afterwards
    pub fn get_chunk(
        &self,
        boundary: &ChunkBoundaries,
        query: &mut Query<(&mut VoxelChunk,)>,
    ) -> Option<VoxelChunk> {
        self.get_chunk_entity(boundary)
            .and_then(|entity| query.get_mut(entity).map(|c| c.0.clone()).ok())
    }

    pub fn new() -> VoxelAccess {
        VoxelAccess {
            chunks: AHashMap::new(),
        }
    }
}
