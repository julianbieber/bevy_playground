use crate::voxel::{Voxel, VoxelTypes};

#[derive(Default, Clone)]
pub struct VoxelPillar {
    pub voxel_heights: Vec<VoxelDescription>,
}

impl VoxelPillar {
    pub fn merge(&mut self) {
        let mut already_removed = 0;

        if self.voxel_heights.len() <= 1 {
            return;
        }
        for i in 1 .. self.voxel_heights.len() {
            let index = i - already_removed;
            let lower_index = index - 1;
            let my = self.voxel_heights[index].lower_voxel;
            let lower = self.voxel_heights[lower_index].upper_voxel;

            if matches!(my, lower) {

            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VoxelDescription {
    pub min: i32,
    pub max: i32,
    pub upper_voxel: Voxel,
    pub lower_voxel: Voxel,
}

impl VoxelDescription {
    pub fn water(min: i32, amount: f32) -> Self {
        let mut full_height = amount.ceil() as i32;
        let highest_fraction = amount.fract();
        
        VoxelDescription { min, max: min + full_height, upper_voxel: Voxel::WaterVoxel { fill: highest_fraction }, lower_voxel: Voxel::WaterVoxel { fill: 1.0 } }
    }

    pub fn solid(min: i32, max: i32, lower_voxel: VoxelTypes, upper_voxel: VoxelTypes) -> Self {
        VoxelDescription {
            min,
            max,
            upper_voxel: Voxel::LandVoxel { typ: upper_voxel },
            lower_voxel: Voxel::LandVoxel { typ: lower_voxel }
        }
    }

    pub fn free(min: i32, max: i32) -> Self {
        VoxelDescription {
            min, max, lower_voxel: Voxel::Nothing, upper_voxel: Voxel::Nothing
        }
    }
}

#[cfg(test)]
mod test {
    use super::{VoxelPillar, VoxelDescription};

    use crate::voxel::Voxel::{LandVoxel, WaterVoxel, Nothing};

    #[test]
    fn voxel_pillars_should_merge_adjacent_water() {
        // voxels of the same type (Land, water, Nothing) should match even if the inner values are different
        let mut pillar = VoxelPillar {
            voxel_heights: vec![
                VoxelDescription::water(0, 1.0),
                VoxelDescription::water(1, 0.6) 
            ],
        };

        pillar.merge();

        assert_eq!(pillar.voxel_heights, vec![
            VoxelDescription::water(0, 1.6)
        ]);

    }

}