use std::sync::Arc;

use bevy::prelude::*;

use crate::voxel_world::voxel::{Voxel, VoxelPosition};

pub struct WorldUpdateResult {
    pub entity_2_mesh: Vec<(Entity, Mesh)>,
    pub voxels_to_replace: Vec<Voxel>,
}

pub struct DelayedWorldTransformations {
    pub transformations: Vec<(Timer, WorldUpdateEvent)>,
}

#[derive(Clone)]
pub struct WorldUpdateEvent {
    pub delete: Arc<dyn Fn() -> Vec<VoxelPosition> + Send + Sync>,
    pub replace: bool,
}
