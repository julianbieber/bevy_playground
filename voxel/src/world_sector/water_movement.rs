use smallvec::SmallVec;

use super::{
    grid::GridWorld,
    pillar::{VoxelDescription, VoxelPillar},
};
use crate::voxel::VoxelRange;

pub fn update_water(world_sector: &mut GridWorld) {
    world_sector.iterate_mut(1, 1, update_single_water_block);
}

fn update_single_water_block(
    center: &mut VoxelPillar,
    left: Option<&mut VoxelPillar>,
    up: Option<&mut VoxelPillar>,
    right: Option<&mut VoxelPillar>,
    down: Option<&mut VoxelPillar>,
) {
    let mut center_changed = false;
    let mut left_changed = false;
    let mut up_changed = false;
    let mut right_changed = false;
    let mut down_changed = false;
    for current_index in 1..center.voxel_heights.len() {
        let lower_index = current_index - 1;

        let lower = center.voxel_heights[lower_index].clone();
        let current = &mut center.voxel_heights[current_index];
        match current.voxel {
            VoxelRange::WaterVoxel { .. } => {
                if lower.max < current.min - 1 {
                    current.min -= 1;
                    current.max -= 1;
                } else {
                }
            }
            _ => (),
        };
    }

    if center_changed {
        center.merge();
    }
}

fn flow_into(center: &mut VoxelDescription, side: &mut VoxelPillar, max_flow: u8) {
    let mut reached_min = false;
    let mut previous_max = center.min;
    let mut inserts: SmallVec<[usize; 8]> = SmallVec::new();
    /*
    for every step on the side
        check if the side step is next to the source

    */
    windows_mut_each(&mut side.voxel_heights, |slice| match slice {
        [lower, higher] => {
            if lower.is_next_to(&center) {
            } else if true {
            }
            true
        }
        [lower] => true, /* center between lower and higher */
        a => {
            dbg!(a);
            panic!("can not happen")
        }
    });
}

fn windows_mut_each<T>(v: &mut [T], mut f: impl FnMut(&mut [T]) -> bool) {
    let mut start = 0;
    let mut end = 1;
    while end < v.len() && f(&mut v[start..start + 1]) {
        start += 1;
        end += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::world_sector::pillar::{VoxelDescription, VoxelPillar};

    use super::flow_into;

    #[test]
    fn flow_into_gap() {
        let mut src = VoxelDescription::water(2, 64);
        let mut dst = VoxelPillar {
            voxel_heights: vec![VoxelDescription::solid(0, 2), VoxelDescription::solid(4, 5)],
        };

        flow_into(&mut src, &mut dst, 32);

        assert_eq!(dst.voxel_heights[1], VoxelDescription::water(3, 32));
        assert_eq!(src, VoxelDescription::water(2, 32));
    }

    #[test]
    fn flow_into_lower_water() {
        let mut src = VoxelDescription::water(2, 64);
        let mut dst = VoxelPillar {
            voxel_heights: vec![VoxelDescription::water(2, 10)],
        };

        flow_into(&mut src, &mut dst, 32);

        assert_eq!(dst.voxel_heights[0], VoxelDescription::water(2, 37));
        assert_eq!(src, VoxelDescription::water(2, 37));
    }

    #[test]
    fn not_flow_into_solid() {
        let mut src = VoxelDescription::water(2, 64);
        let mut dst = VoxelPillar {
            voxel_heights: vec![VoxelDescription::solid(2, 3)],
        };

        flow_into(&mut src, &mut dst, 32);

        assert_eq!(dst.voxel_heights[0], VoxelDescription::solid(2, 3));
        assert_eq!(src, VoxelDescription::water(2, 64));
    }

    #[test]
    fn flow_above() {
        let mut src = VoxelDescription::water(2, 64);
        let mut dst = VoxelPillar {
            voxel_heights: vec![VoxelDescription::solid(1, 2)],
        };

        flow_into(&mut src, &mut dst, 32);

        assert_eq!(dst.voxel_heights[1], VoxelDescription::water(2, 32));
        assert_eq!(src, VoxelDescription::water(2, 32));
    }

    #[test]
    fn flow_below() {
        let mut src = VoxelDescription::water(2, 32);
        let mut dst = VoxelPillar {
            voxel_heights: vec![VoxelDescription::solid(3, 4)],
        };

        flow_into(&mut src, &mut dst, 10);

        assert_eq!(dst.voxel_heights[0], VoxelDescription::water(2, 10));
        assert_eq!(src, VoxelDescription::water(2, 22));
    }

    #[test]
    fn flow_into_empty() {
        let mut src = VoxelDescription::water(2, 32);
        let mut dst = VoxelPillar {
            voxel_heights: vec![],
        };

        flow_into(&mut src, &mut dst, 16);

        assert_eq!(dst.voxel_heights[0], VoxelDescription::water(2, 16));
        assert_eq!(src, VoxelDescription::water(2, 16));
    }

    #[test]
    fn flow_with_overlap() {
        let mut src = VoxelDescription::water(2, 164);
        let mut dst = VoxelPillar {
            voxel_heights: vec![VoxelDescription::solid(2, 3)],
        };

        flow_into(&mut src, &mut dst, 10);

        assert_eq!(dst.voxel_heights[1], VoxelDescription::water(3, 10));
        assert_eq!(src, VoxelDescription::water(2, 154));
    }

    #[test]
    fn flow_into_overlapped_gap() {
        let mut src = VoxelDescription::water(1, 320);
        let mut dst = VoxelPillar {
            voxel_heights: vec![VoxelDescription::solid(0, 2), VoxelDescription::solid(4, 5)],
        };

        flow_into(&mut src, &mut dst, 128);

        assert_eq!(dst.voxel_heights[1], VoxelDescription::water(3, 128));
        assert_eq!(src, VoxelDescription::water(1, 192));
    }
}
