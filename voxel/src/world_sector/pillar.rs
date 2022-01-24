use crate::voxel::VoxelRange;

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
        for i in 1..self.voxel_heights.len() {
            let index = i - already_removed;
            let lower_index = index - 1;
            let my = self.voxel_heights[index].clone();
            let lower = &mut self.voxel_heights[lower_index];
            if lower.try_add(&my) {
                self.voxel_heights.remove(index);
                already_removed += 1;
            }
        }
    }
}

/// min and max are inclusive
#[derive(Debug, Clone, PartialEq)]
pub struct VoxelDescription {
    pub min: i32,
    pub max: i32,
    pub voxel: VoxelRange,
}

impl VoxelDescription {
    pub fn water(min: i32, amount: f32) -> Self {
        let full_height = amount.ceil() as i32;
        let highest_fraction = (amount.fract() * 128.0) as u8;
        VoxelDescription {
            min,
            max: min + full_height - 1,
            voxel: VoxelRange::WaterVoxel {
                upper_fill: highest_fraction + ((highest_fraction == 0) as u8) * 128,
            },
        }
    }

    pub fn solid(min: i32, max: i32) -> Self {
        VoxelDescription {
            min,
            max,
            voxel: VoxelRange::LandVoxel {},
        }
    }

    pub fn try_add(&mut self, other: &VoxelDescription) -> bool {
        //dbg!(&self, &other);
        match (&mut self.voxel, other.voxel) {
            (VoxelRange::LandVoxel {}, VoxelRange::LandVoxel {}) => {
                if self.max == other.min - 1 {
                    self.max = other.max;
                    true
                } else {
                    false
                }
            }
            (
                VoxelRange::WaterVoxel { upper_fill: fill_1 },
                VoxelRange::WaterVoxel { upper_fill: fill_2 },
            ) => {
                if self.max == other.min - 1 {
                    let sum = *fill_1 + fill_2;
                    self.max = other.max - ((sum < 128) as i32);
                    (*fill_1) = sum - (((sum > 128) as u8) << 7);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_next_to(&self, other: &VoxelDescription) -> bool {
        // self overlapping higher
        between(self.min, other) ||
        // self overlapping lower
        between(self.max, other) ||
        between(other.min, self) ||
        between(other.max, self)
        // other between self
        // self between other
    }

    pub fn insert(&mut self, amount: u8) -> u8 {
        match &mut (self.voxel) {
            VoxelRange::LandVoxel {} => amount,
            VoxelRange::WaterVoxel { ref mut upper_fill } => {
                let actual_amount = (255 - ((*upper_fill) as i32 + amount as i32).min(255)) as u8;
                *upper_fill += actual_amount;
                amount - actual_amount
            }
        }
    }
}

#[inline(always)]
fn between(a: i32, vd: &VoxelDescription) -> bool {
    a >= vd.min && a <= vd.max
}

#[cfg(test)]
mod test {
    use crate::voxel::VoxelRange;

    use super::{VoxelDescription, VoxelPillar};

    #[test]
    fn voxel_pillars_should_merge_adjacent_water() {
        // voxels of the same type (Land, water, Nothing) should match even if the inner values are different
        let mut pillar = VoxelPillar {
            voxel_heights: vec![
                VoxelDescription::water(0, 1.0),
                VoxelDescription::water(1, 0.6),
            ],
        };

        dbg!(&pillar.voxel_heights);

        pillar.merge();

        assert_eq!(pillar.voxel_heights, vec![VoxelDescription::water(0, 1.6)]);
    }

    #[test]
    fn merge_water_with_overflow() {
        let mut pillar = VoxelPillar {
            voxel_heights: vec![
                VoxelDescription::water(0, 1.4),
                VoxelDescription::water(2, 1.7),
            ],
        };

        pillar.merge();

        assert_eq!(pillar.voxel_heights, vec![VoxelDescription::water(0, 3.1)]);
    }

    #[test]
    fn merge_water_combined() {
        let mut pillar = VoxelPillar {
            voxel_heights: vec![
                VoxelDescription::water(0, 1.4),
                VoxelDescription::water(2, 0.2),
            ],
        };

        pillar.merge();

        assert_eq!(pillar.voxel_heights, vec![VoxelDescription::water(0, 1.6)]);
    }

    #[test]
    fn initialize_water() {
        assert_eq!(
            VoxelDescription::water(0, 1.0),
            VoxelDescription {
                min: 0,
                max: 0,
                voxel: VoxelRange::WaterVoxel { upper_fill: 128 }
            }
        );

        assert_eq!(
            VoxelDescription::water(0, 1.1),
            VoxelDescription {
                min: 0,
                max: 1,
                voxel: VoxelRange::WaterVoxel { upper_fill: 12 }
            }
        );

        assert_eq!(
            VoxelDescription::water(0, 0.1),
            VoxelDescription {
                min: 0,
                max: 0,
                voxel: VoxelRange::WaterVoxel { upper_fill: 12 }
            }
        );
    }

    #[test]
    fn insert_water_do_not_overflow() {
        let mut voxel = VoxelDescription::water(0, 1.0);
        assert_eq!(voxel.insert(255), 255);
    }

    #[test]
    fn insert_water() {
        let mut voxel = VoxelDescription::water(0, 0.5);
        assert_eq!(voxel.insert(10), 0);
    }

    #[test]
    fn insert_water_slight_overflow() {
        let mut voxel = VoxelDescription::water(0, 0.5);
        assert_eq!(voxel.insert(165), 37);
    }
}
