use ahash::AHashMap;

type YWorldCoordinate = AHashMap<i32, Voxel>;
type ZWorldCoordinate = AHashMap<i32, YWorldCoordinate>;

pub type WorldStructure = AHashMap<i32, ZWorldCoordinate>;

#[derive(Clone, Debug)]
pub struct Voxel {
    pub position: (i32, i32, i32),
    pub typ: VoxelTypes,
}

#[derive(Clone, Debug)]
pub enum VoxelTypes {
    Rock,
}

pub trait WorldStrucutureImpl {
    fn add_voxel(&mut self, voxel: Voxel);
    fn get_at(&self, x: &i32, y: &i32, z: &i32) -> Option<&Voxel>;
}

impl WorldStrucutureImpl for WorldStructure {
    fn add_voxel(&mut self, voxel: Voxel) {
        let x = voxel.position.0;
        let y = voxel.position.1;
        let z = voxel.position.2;
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
}
