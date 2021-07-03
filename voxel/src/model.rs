use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    access::VoxelAccess,
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
    pub delete: Arc<dyn Fn(&VoxelAccess) -> Vec<VoxelPosition> + Send + Sync>,
    pub replace: bool,
}
