use super::voxel::{Voxel, VoxelPosition};
use ahash::AHashMap;

type YWorldCoordinate = AHashMap<i32, Voxel>;
type ZWorldCoordinate = AHashMap<i32, YWorldCoordinate>;

pub type WorldStructure = AHashMap<i32, ZWorldCoordinate>;

pub struct Terrain {
    pub structure: WorldStructure,
}

pub trait WorldStrucutureImpl {
    fn add_voxel(&mut self, voxel: Voxel);
    fn get_at(&self, x: &i32, y: &i32, z: &i32) -> Option<&Voxel>;
    fn get_at_voxel(&self, voxel: &VoxelPosition) -> Option<&Voxel>;
}

impl WorldStrucutureImpl for WorldStructure {
    fn add_voxel(&mut self, voxel: Voxel) {
        let x = voxel.position.x;
        let y = voxel.position.y;
        let z = voxel.position.z;
        match self.get_mut(&x) {
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
                self.insert(x, z_map);
            }
        };
    }

    fn get_at(&self, x: &i32, y: &i32, z: &i32) -> Option<&Voxel> {
        self.get(x)
            .map(|i| i.get(z).map(|i| i.get(y)))
            .flatten()
            .flatten()
    }

    fn get_at_voxel(&self, voxel: &VoxelPosition) -> Option<&Voxel> {
        self.get_at(&voxel.x, &voxel.y, &voxel.z)
    }
}
