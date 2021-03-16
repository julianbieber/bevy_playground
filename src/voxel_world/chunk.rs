use ahash::AHashMap;

use crate::voxel_world::voxel::{Voxel, VoxelPosition};

use super::voxel::VoxelTypes;

#[derive(Clone)]
pub struct VoxelChunk {
    voxels: AHashMap<i32, AHashMap<i32, AHashMap<i32, VoxelTypes>>>,
    pub count: usize,
}

impl VoxelChunk {
    pub fn empty() -> VoxelChunk {
        VoxelChunk {
            voxels: AHashMap::new(),
            count: 0,
        }
    }

    pub fn get_voxels(&self) -> Vec<Voxel> {
        let mut voxels = Vec::with_capacity(self.count);
        for (x, xs) in self.voxels.iter() {
            for (y, ys) in xs.iter() {
                for (z, voxel) in ys.iter() {
                    voxels.push(Voxel {
                        position: VoxelPosition {
                            x: x.clone(),
                            y: y.clone(),
                            z: z.clone(),
                        },
                        typ: voxel.clone(),
                    });
                }
            }
        }

        voxels
    }

    pub fn set(&mut self, voxel: Voxel) {
        if let None = self
            .voxels
            .entry(voxel.position.x)
            .or_insert(AHashMap::new())
            .entry(voxel.position.y)
            .or_insert(AHashMap::new())
            .insert(voxel.position.z, voxel.typ)
        {
            self.count += 1;
        }
    }

    pub fn remove(&mut self, position: VoxelPosition) -> Option<Voxel> {
        let old = self
            .voxels
            .entry(position.x)
            .or_insert(AHashMap::new())
            .entry(position.y)
            .or_insert(AHashMap::new())
            .remove(&position.z);
        if old.is_some() {
            self.count -= 1;
        }
        old.map(|typ| Voxel { position, typ })
    }

    pub fn get(&self, position: &VoxelPosition) -> Option<&VoxelTypes> {
        self.voxels
            .get(&position.x)
            .and_then(|ys| ys.get(&position.y))
            .and_then(|zs| zs.get(&position.z))
    }
}
