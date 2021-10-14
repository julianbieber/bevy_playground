use std::sync::Arc;

use bevy::prelude::*;

use crate::{voxel::VoxelPosition, world_sector::DefaultWorldSector};

pub struct WorldUpdateResult {
    pub entity_2_mesh: Vec<(Entity, Mesh)>,
    pub voxels_to_replace: Vec<VoxelPosition>,
}

pub struct DelayedWorldTransformations {
    pub transformations: Vec<(Timer, WorldUpdateEvent)>,
}

#[derive(Clone)]
pub struct WorldUpdateEvent {
    pub delete: Arc<dyn Fn(&DefaultWorldSector) -> Vec<VoxelPosition> + Send + Sync>,
    pub replace: bool,
}