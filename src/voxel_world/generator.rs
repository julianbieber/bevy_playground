use super::voxel::{Voxel, VoxelTypes};
use ahash::AHashMap;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use flume::Sender;
use lerp::Lerp;
use rand::prelude::*;
use rand::seq::SliceRandom;

use super::world_structure::*;

pub struct VoxelWorld {
    pub pillars: Vec<PillarGenerator>,
}

impl VoxelWorld {
    pub fn generate(width: i32, depth: i32, mut rng: SmallRng) -> VoxelWorld {
        let pillars: Vec<_> = (0..10)
            .into_iter()
            .map(|_| PillarGenerator::new(&mut rng, width, depth))
            .collect();
        VoxelWorld { pillars }
    }

    pub fn add_to_world(
        self,
        pool: ResMut<AsyncComputeTaskPool>,
        tx: Res<Sender<(Mesh, Terrain, Option<Entity>)>>,
    ) {
        for pillar in self.pillars.into_iter() {
            let tx_copy = tx.clone();
            pool.spawn(async move {
                let terrain = pillar.voxels();
                let m = Mesh::from(&terrain);
                tx_copy.send((m, terrain, None)).unwrap();
            })
            .detach();
        }
    }
}

pub struct PillarGenerator {
    position: (i32, i32),
    height: i32,
    upper_radius: i32,
    mid_radius: i32,
    lower_radius: i32,
    rock_types: Vec<VoxelTypes>,
}

impl PillarGenerator {
    fn new(rng: &mut SmallRng, width_boundary: i32, depth_boundary: i32) -> PillarGenerator {
        PillarGenerator {
            position: (
                rng.gen_range(width_boundary / -2..width_boundary / 2),
                rng.gen_range(depth_boundary / -2..depth_boundary / 2),
            ),
            height: rng.gen_range(10..20),
            upper_radius: rng.gen_range(10..20),
            mid_radius: rng.gen_range(5..20),
            lower_radius: rng.gen_range(10..21),
            rock_types: vec![
                VoxelTypes::DarkRock1,
                VoxelTypes::DarkRock2,
                VoxelTypes::LightRock1,
                VoxelTypes::LightRock2,
                VoxelTypes::CrackedRock,
            ],
        }
    }

    pub fn voxels(&self) -> Terrain {
        let mut rng = SmallRng::from_entropy();
        let mut world: WorldStructure = AHashMap::new();
        for layer in 0..self.height {
            let radius = self.radius_at_level(layer);
            let radius_sq = radius * radius;
            for x in (self.position.0 - radius)..(self.position.0 + radius) {
                for z in (self.position.1 - radius)..(self.position.1 + radius) {
                    let distance_sq = (self.position.0 - x) * (self.position.0 - x)
                        + (self.position.1 - z) * (self.position.1 - z);
                    if distance_sq <= radius_sq {
                        let voxel = Voxel::new(x, layer, z, self.voxel_type(&mut rng, layer));
                        world.add_voxel(voxel);
                    }
                }
            }
        }
        Terrain { structure: world }
    }

    fn voxel_type(&self, mut rng: &mut SmallRng, y: i32) -> VoxelTypes {
        if y == self.height - 1 {
            VoxelTypes::Moss
        } else if y == 0 {
            VoxelTypes::Lava
        } else {
            self.rock_types.choose(&mut rng).unwrap().clone()
        }
    }

    fn radius_at_level(&self, level: i32) -> i32 {
        if level < 0 || level > self.height {
            0
        } else if level == 0 {
            self.lower_radius
        } else if level == self.height {
            self.upper_radius
        } else if level < self.height / 2 {
            (self.lower_radius as f32).lerp(
                self.mid_radius as f32,
                level as f32 / (self.height as f32 / 2.0),
            ) as i32
        } else {
            (self.mid_radius as f32).lerp(
                self.upper_radius as f32,
                (level as f32 - self.height as f32 / 2.0f32) / (self.height as f32 / 2.0),
            ) as i32
        }
    }
}
