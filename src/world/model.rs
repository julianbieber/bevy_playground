use std::sync::Arc;

use bevy::prelude::*;

use crate::voxel_world::{
    voxel::{Voxel, VoxelPosition},
    world_structure::Terrain,
};

pub struct WorldUpdateResult {
    pub new_terrain_mesh: Mesh,
    pub terrain: Terrain,
    pub existing_terrain_entity: Option<Entity>,
    pub voxels_to_replace: Vec<Voxel>,
}

pub struct DelayedWorldTransformations {
    pub transformations: Vec<(Timer, WorldUpdateEvent)>,
}

#[derive(Clone)]
pub struct WorldUpdateEvent {
    pub entity: Entity,
    pub delete: Arc<dyn Fn(&Terrain) -> Vec<VoxelPosition> + Send + Sync>,
    pub replace: bool,
}
