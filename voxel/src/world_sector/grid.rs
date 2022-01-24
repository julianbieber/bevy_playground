use std::cell::{Ref, RefCell, RefMut};

use itertools::iproduct;

use crate::{
    boundaries::ChunkBoundaries,
    voxel::{VoxelDirection, VoxelPosition},
    world_sector::index_magic::{left_or_down, right_or_up},
};

use super::{
    consts::{L1DIMENSION, L1DIMENSION_I, L2DIMENSION, L2DIMENSION_I, L3DIMENSION, L3DIMENSION_I},
    pillar::VoxelPillar,
};

const L3GRID_ENTRY_SIZE: i32 = L1DIMENSION_I * L2DIMENSION_I;

pub type GridWorld = L3Grid;

#[derive(Default)]
pub struct L3Grid {
    data: Box<[[L2Grid; L3DIMENSION]; L3DIMENSION]>,
    pub min: [i32; 2],
}

#[derive(Default)]
struct L2Grid {
    pub data: Box<[[L1Grid; L2DIMENSION]; L2DIMENSION]>,
    pub min: [i32; 2],
}

#[derive(Default)]
struct L1Grid {
    pub data: Box<[[VoxelPillar; L1DIMENSION]; L1DIMENSION]>,
    pub min: [i32; 2],
}

impl L3Grid {
    pub fn empty(center: [i32; 2]) -> Self {
        // The center of the 8x8 chunks is at position 4,4
        let center_boundary =
            ChunkBoundaries::<L3GRID_ENTRY_SIZE>::aligned([center[0], 0, center[1]].into());
        let lower_left =
            center_boundary.in_direction([-L3DIMENSION_I / 2, 0, -L3DIMENSION_I / 2].into());
        let mut grid = L3Grid {
            min: [lower_left.min.x, lower_left.min.z],
            ..Default::default()
        };

        grid.init_inner_mins();

        grid
    }
    fn init_inner_mins(&mut self) {
        for x in 0..L3DIMENSION {
            for z in 0..L3DIMENSION {
                self.data[x][z].min = [
                    self.min[0] + x as i32 * L3GRID_ENTRY_SIZE,
                    self.min[0] + z as i32 * L3GRID_ENTRY_SIZE,
                ];
                self.data[x][z].init_inner_mins();
            }
        }
    }

    /// The provided surrounding for the pillar is ordered as follows: left, behind, right, front
    ///   1
    /// 0 X 2
    ///   3
    pub fn iterate<F>(&self, x3: usize, z3: usize, f: F)
    where
        F: FnMut(
            &VoxelPillar,
            Option<&VoxelPillar>,
            Option<&VoxelPillar>,
            Option<&VoxelPillar>,
            Option<&VoxelPillar>,
        ),
    {
        unsafe { self.iterate_unchecked(x3, z3, f) }
    }

    unsafe fn iterate_unchecked<F>(&self, x3: usize, z3: usize, mut f: F)
    where
        F: FnMut(
            &VoxelPillar,
            Option<&VoxelPillar>,
            Option<&VoxelPillar>,
            Option<&VoxelPillar>,
            Option<&VoxelPillar>,
        ),
    {
        let l2 = &self.data.get_unchecked(x3).get_unchecked(z3);
        for (x2, z2) in iproduct!(0..L2DIMENSION, 0..L2DIMENSION) {
            let l1 = &l2.data.get_unchecked(x2).get_unchecked(z2);
            for (x1, z1) in iproduct!(0..L1DIMENSION, 0..L1DIMENSION) {
                let center = &l1.data.get_unchecked(x1).get_unchecked(z1);
                f(
                    center,
                    left_or_down(x3, x2, x1).map(|(sx3, sx2, sx1)| {
                        self.data
                            .get_unchecked(sx3)
                            .get_unchecked(z3)
                            .data
                            .get_unchecked(sx2)
                            .get_unchecked(z2)
                            .data
                            .get_unchecked(sx1)
                            .get_unchecked(z1)
                    }),
                    right_or_up(z3, z2, z1).map(|(sz3, sz2, sz1)| {
                        self.data
                            .get_unchecked(x3)
                            .get_unchecked(sz3)
                            .data
                            .get_unchecked(x2)
                            .get_unchecked(sz2)
                            .data
                            .get_unchecked(x1)
                            .get_unchecked(sz1)
                    }),
                    right_or_up(x3, x2, x1).map(|(sx3, sx2, sx1)| {
                        self.data
                            .get_unchecked(sx3)
                            .get_unchecked(z3)
                            .data
                            .get_unchecked(sx2)
                            .get_unchecked(z2)
                            .data
                            .get_unchecked(sx1)
                            .get_unchecked(z1)
                    }),
                    left_or_down(z3, z2, z1).map(|(sz3, sz2, sz1)| {
                        self.data
                            .get_unchecked(x3)
                            .get_unchecked(sz3)
                            .data
                            .get_unchecked(x2)
                            .get_unchecked(sz2)
                            .data
                            .get_unchecked(x1)
                            .get_unchecked(sz1)
                    }),
                );
            }
        }
    }

    pub fn iterate_mut<F>(&mut self, x3: usize, z3: usize, f: F)
    where
        F: FnMut(
            &mut VoxelPillar,
            Option<&mut VoxelPillar>,
            Option<&mut VoxelPillar>,
            Option<&mut VoxelPillar>,
            Option<&mut VoxelPillar>,
        ),
    {
        unsafe {
            self.iterate_mut_unchecked(x3, z3, f);
        }
    }

    unsafe fn iterate_mut_unchecked<F>(&mut self, x3: usize, z3: usize, mut f: F)
    where
        F: FnMut(
            &mut VoxelPillar,
            Option<&mut VoxelPillar>,
            Option<&mut VoxelPillar>,
            Option<&mut VoxelPillar>,
            Option<&mut VoxelPillar>,
        ),
    {
        for (x2, z2) in iproduct!(0..L2DIMENSION, 0..L2DIMENSION) {
            for (x1, z1) in iproduct!(0..L1DIMENSION, 0..L1DIMENSION) {
                let center = self
                    .data
                    .get_unchecked_mut(x3)
                    .get_unchecked_mut(z3)
                    .data
                    .get_unchecked_mut(x2)
                    .get_unchecked_mut(z2)
                    .data
                    .get_unchecked_mut(x1)
                    .as_mut_ptr()
                    .add(z1);

                f(
                    &mut *center,
                    left_or_down(x3, x2, x1).map(|(sx3, sx2, sx1)| {
                        &mut *self
                            .data
                            .get_unchecked_mut(sx3)
                            .get_unchecked_mut(z3)
                            .data
                            .get_unchecked_mut(sx2)
                            .get_unchecked_mut(z2)
                            .data
                            .get_unchecked_mut(sx1)
                            .as_mut_ptr()
                            .add(z1)
                    }),
                    right_or_up(z3, z2, z1).map(|(sz3, sz2, sz1)| {
                        &mut *self
                            .data
                            .get_unchecked_mut(x3)
                            .get_unchecked_mut(sz3)
                            .data
                            .get_unchecked_mut(x2)
                            .get_unchecked_mut(sz2)
                            .data
                            .get_unchecked_mut(x1)
                            .as_mut_ptr()
                            .add(sz1)
                    }),
                    right_or_up(x3, x2, x1).map(|(sx3, sx2, sx1)| {
                        &mut *self
                            .data
                            .get_unchecked_mut(sx3)
                            .get_unchecked_mut(z3)
                            .data
                            .get_unchecked_mut(sx2)
                            .get_unchecked_mut(z2)
                            .data
                            .get_unchecked_mut(sx1)
                            .as_mut_ptr()
                            .add(z1)
                    }),
                    left_or_down(z3, z2, z1).map(|(sz3, sz2, sz1)| {
                        &mut *self
                            .data
                            .get_unchecked_mut(x3)
                            .get_unchecked_mut(sz3)
                            .data
                            .get_unchecked_mut(x2)
                            .get_unchecked_mut(sz2)
                            .data
                            .get_unchecked_mut(x1)
                            .as_mut_ptr()
                            .add(sz1)
                    }),
                );
            }
        }
    }
}

impl L2Grid {
    fn init_inner_mins(&mut self) {
        for x in 0..L2DIMENSION {
            for z in 0..L2DIMENSION {
                self.data[x][z].min = [self.min[0] + x as i32 * 32, self.min[0] + z as i32 * 32];
            }
        }
    }
}

pub struct SimpleVec {
    data: Vec<VoxelPillar>,
}

impl SimpleVec {
    pub fn empty() -> SimpleVec {
        SimpleVec {
            data: vec![
                VoxelPillar::default();
                L3DIMENSION.pow(2) * L2DIMENSION.pow(2) * L1DIMENSION.pow(2)
            ],
        }
    }

    pub fn iterate(
        &self,
        f: fn(
            &VoxelPillar,
            Option<&VoxelPillar>,
            Option<&VoxelPillar>,
            Option<&VoxelPillar>,
            Option<&VoxelPillar>,
        ),
    ) {
        self.data.iter().for_each(|p| f(p, None, None, None, None));
    }
}

#[cfg(test)]
mod tests {
    use crate::world_sector::{
        consts::{L1DIMENSION_I, L2DIMENSION_I, L3DIMENSION_I},
        grid::L3Grid,
    };

    #[test]
    fn initialize_minima() {
        let grid = L3Grid::empty([1, 1]);

        assert_eq!(
            grid.min,
            [
                -L3DIMENSION_I / 2 * L2DIMENSION_I * L1DIMENSION_I,
                -L3DIMENSION_I / 2 * L2DIMENSION_I * L1DIMENSION_I
            ]
        );
    }
}
