use std::sync::Arc;

use bevy::prelude::*;

use crate::voxel_world::{
    chunk::{ChunkBoundaries, VoxelChunk},
    voxel::{Voxel, VoxelPosition},
};

pub struct WorldUpdateResult {
    pub entity_2_mesh: Vec<(Entity, Mesh)>,
    pub voxels_to_replace: Vec<Voxel>,
}

pub struct DelayedWorldTransformations {
    pub transformations: Vec<(Timer, WorldUpdateEvent)>,
}

#[derive(Clone)]
pub struct WorldUpdateEvent {
    /// can be optimized further by prefiltering the ChunkBoundaries for the highstorm (only selecting boundaries that contain voxels) and only chose the first chunk that contains voxels at the top layer
    pub chunk_filter: Vec<ChunkBoundaries>,
    pub delete: Arc<dyn Fn(&Vec<VoxelChunk>) -> Vec<VoxelPosition> + Send + Sync>,
    pub replace: bool,
}
