use super::voxel::{Voxel, VoxelPosition};
use ahash::AHashMap;
use bevy::prelude::Vec3;

type YWorldCoordinate = AHashMap<i32, Voxel>;
type ZWorldCoordinate = AHashMap<i32, YWorldCoordinate>;

pub type WorldStructure = AHashMap<i32, ZWorldCoordinate>;

#[derive(Clone, Debug)]
pub struct Terrain {
    pub structure: WorldStructure,
    pub min: [i32; 3],
    pub max: [i32; 3],
    min_count: [u32; 3],
    max_count: [u32; 3],
}

impl Terrain {
    pub fn new() -> Terrain {
        Terrain {
            structure: AHashMap::new(),
            min: [0, 0, 0],
            min_count: [0, 0, 0],
            max: [0, 0, 0],
            max_count: [0, 0, 0],
        }
    }

    pub fn recalculate(&mut self) {
        self.min = [i32::max_value(), i32::max_value(), i32::max_value()];
        self.min_count = [0, 0, 0];
        self.max = [i32::min_value(), i32::min_value(), i32::min_value()];
        self.max_count = [0, 0, 0];
        for (x, i) in self.structure.iter() {
            if *x < self.min[0] {
                self.min[0] = *x;
                self.min_count[0] = 1;
            } else if *x == self.min[0] {
                self.min_count[0] += 1;
            }

            if *x > self.max[0] {
                self.max[0] = *x;
                self.max_count[0] = 1;
            } else if *x == self.max[0] {
                self.max_count[0] += 1;
            }

            for (z, i) in i.iter() {
                if *z < self.min[2] {
                    self.min[2] = *z;
                    self.min_count[2] = 1;
                } else if *z == self.min[2] {
                    self.min_count[2] += 1;
                }

                if *z > self.max[2] {
                    self.max[2] = *z;
                    self.max_count[2] = 1;
                } else if *z == self.max[2] {
                    self.max_count[2] += 1;
                }

                for (y, _) in i.iter() {
                    if *y < self.min[1] {
                        self.min[1] = *y;
                        self.min_count[1] = 1;
                    } else if *y == self.min[1] {
                        self.min_count[1] += 1;
                    }

                    if *y > self.max[1] {
                        self.max[1] = *y;
                        self.max_count[1] = 1;
                    } else if *y == self.max[1] {
                        self.max_count[1] += 1;
                    }
                }
            }
        }
    }

    pub fn add_voxel(&mut self, voxel: Voxel) {
        let x = voxel.position.x;
        let y = voxel.position.y;
        let z = voxel.position.z;
        match self.structure.get_mut(&x) {
            Some(z_map) => match z_map.get_mut(&z) {
                Some(y_map) => {
                    y_map.insert(y, voxel);
                }
                None => {
                    let mut y_map = AHashMap::new();
                    y_map.insert(y, voxel);
                    z_map.insert(z, y_map);
                }
            },
            None => {
                let mut z_map = AHashMap::new();
                let mut y_map = AHashMap::new();
                y_map.insert(y, voxel);
                z_map.insert(z, y_map);
                self.structure.insert(x, z_map);
            }
        };
    }

    pub fn get_at(&self, x: &i32, y: &i32, z: &i32) -> Option<&Voxel> {
        self.structure
            .get(x)
            .map(|i| i.get(z).map(|i| i.get(y)))
            .flatten()
            .flatten()
    }

    pub fn get_at_voxel(&self, voxel: &VoxelPosition) -> Option<&Voxel> {
        self.get_at(&voxel.x, &voxel.y, &voxel.z)
    }

    pub fn is_surrounded(&self, voxel: &VoxelPosition) -> bool {
        for x in [voxel.x - 1, voxel.x + 1].iter() {
            for y in [voxel.y - 1, voxel.y + 1].iter() {
                for z in [voxel.z - 1, voxel.z + 1].iter() {
                    if self.get_at(&x, &y, &z).is_none() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn remove_voxel(&mut self, voxel: VoxelPosition) {
        self.structure.get_mut(&voxel.x).map(|i| {
            i.get_mut(&voxel.z).map(|i| {
                i.remove(&voxel.y);
            });
        });
    }
}
