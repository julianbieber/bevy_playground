mod height;
mod noise_sampler;
mod type_decision;

use std::ops::Range;

use rand::prelude::*;
use smallvec::SmallVec;

use crate::voxel::VoxelTypes;

use self::{height::HeightGen, type_decision::VoxelTypeDecision};

pub struct Generator {
    height_gen: HeightGen,
    type_decision: VoxelTypeDecision,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            height_gen: HeightGen::new(),
            type_decision: VoxelTypeDecision::default(),
        }
    }

    pub fn generate_chunk<const I: usize>(
        &self,
        x: i32,
        y_range: Range<i32>,
        z: i32,
    ) -> SmallVec<[VoxelTypes; I]> {
        let total_y = self.height_gen.get_height_factor(x, z);
        let mut voxels = SmallVec::new();
        let mut rng = SmallRng::from_entropy();
        for y in y_range {
            if y < total_y {
                voxels.push(
                    self.type_decision
                        .get_type(&mut rng, x, y, z, y >= total_y - 1),
                );
            }
        }
        voxels
    }
}
