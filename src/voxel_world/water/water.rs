use std::collections::VecDeque;

use ahash::{AHashMap, AHashSet};

use crate::voxel_world::voxel::VoxelPosition;
pub(super) const WATER_QUADS: usize = 1024;

pub struct Water {
    pub(super) voxels: AHashMap<VoxelPosition, WaterVoxel>,
    pub(super) added: AHashSet<VoxelPosition>,
    pub(super) removed: AHashSet<VoxelPosition>,
    pub(super) unused: VecDeque<[u32; 4]>,
}

/*
Fixed index buffer builds quads from all vertices.
When adding/removing values, only Vertex positions and normals have to be updated.
 */
pub(super) struct WaterVoxel {
    pub(super) indices: Vec<[u32; 4]>,
}

impl Water {
    pub fn new() -> Water {
        let mut unused = VecDeque::with_capacity(WATER_QUADS);
        for i in 0..WATER_QUADS {
            unused.push_back([
                i as u32 * 4 + 0,
                i as u32 * 4 + 1,
                i as u32 * 4 + 2,
                i as u32 * 4 + 3,
            ]);
        }
        Water {
            voxels: AHashMap::new(),
            added: AHashSet::new(),
            removed: AHashSet::new(),
            unused,
        }
    }

    pub fn add(&mut self, p: VoxelPosition) {
        self.added.insert(p);
    }

    pub fn remove(&mut self, p: VoxelPosition) {
        self.removed.insert(p);
    }
}
