use ahash::AHashMap;
use bevy::prelude::Entity;
use std::collections::hash_map::{Iter, IterMut};

use super::{
    boundaries::ChunkBoundaries,
    chunk::VoxelChunk,
    voxel::{VoxelPosition, VoxelTypes},
};

pub struct VoxelAccess {
    chunks: AHashMap<ChunkBoundaries, (Entity, VoxelChunk)>,
}

impl VoxelAccess {
    pub fn get_chunk_containing(&self, position: VoxelPosition) -> Option<&VoxelChunk> {
        let boundary = ChunkBoundaries::aligned(position);

        self.chunks.get(&boundary).map(|(_, c)| c)
    }

    pub fn get_chunk_entity_containing(
        &self,
        position: VoxelPosition,
    ) -> Option<&(Entity, VoxelChunk)> {
        let boundary = ChunkBoundaries::aligned(position);

        self.chunks.get(&boundary)
    }

    pub fn get_chunk_containing_mut(&mut self, position: VoxelPosition) -> Option<&mut VoxelChunk> {
        let boundary = ChunkBoundaries::aligned(position);

        self.chunks.get_mut(&boundary).map(|(_, c)| c)
    }

    pub fn get_chunk(&self, boundary: &ChunkBoundaries) -> Option<&VoxelChunk> {
        self.chunks.get(boundary).map(|(_, c)| c)
    }

    pub fn get_chunk_entity(&self, boundary: &ChunkBoundaries) -> Option<&(Entity, VoxelChunk)> {
        self.chunks.get(boundary)
    }

    pub fn add_chunk(&mut self, boundary: ChunkBoundaries, entity: Entity, chunk: VoxelChunk) {
        self.chunks.insert(boundary, (entity, chunk));
    }

    pub fn new() -> VoxelAccess {
        VoxelAccess {
            chunks: AHashMap::new(),
        }
    }

    pub fn iter(&self) -> Iter<'_, ChunkBoundaries, (Entity, VoxelChunk)> {
        self.chunks.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, ChunkBoundaries, (Entity, VoxelChunk)> {
        self.chunks.iter_mut()
    }

    pub fn get_voxel(&self, position: VoxelPosition) -> Option<VoxelTypes> {
        self.get_chunk_containing(position)
            .and_then(|c| c.get(&position))
    }
}
