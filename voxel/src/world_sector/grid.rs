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
    pub data: [[RefCell<VoxelPillar>; L1DIMENSION]; L1DIMENSION],
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
    pub fn iterate<F>(&self, mut f: F)
    where
        F: FnMut(
            Ref<VoxelPillar>,
            Option<Ref<VoxelPillar>>,
            Option<Ref<VoxelPillar>>,
            Option<Ref<VoxelPillar>>,
            Option<Ref<VoxelPillar>>,
        ),
    {
        for (x3, z3) in iproduct!(0..L3DIMENSION, 0..L3DIMENSION) {
            let l2 = &self.data[x3][z3];
            for (x2, z2) in iproduct!(0..L2DIMENSION, 0..L2DIMENSION) {
                let l1 = &l2.data[x2][z2];
                for (x1, z1) in iproduct!(0..L1DIMENSION, 0..L1DIMENSION) {
                    let center = l1.data[x1][z1].borrow();
                    f(
                        center,
                        left_or_down(x3, x2, x1)
                            .map(|(sx3, sx2, sx1)| self.data[sx3][z3].data[sx2][z2].data[sx1][z1].borrow()),
                        right_or_up(z3, z2, z1)
                            .map(|(sz3, sz2, sz1)| self.data[x3][sz3].data[x2][sz2].data[x1][sz1].borrow()),
                        right_or_up(x3, x2, x1)
                            .map(|(sx3, sx2, sx1)| self.data[sx3][z3].data[sx2][z2].data[sx1][z1].borrow()),
                        left_or_down(z3, z2, z1)
                            .map(|(sz3, sz2, sz1)| self.data[x3][sz3].data[x2][sz2].data[x1][sz1].borrow()),
                    );
                }
            }
        }
    }

    pub fn iterate_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(
            RefMut<VoxelPillar>,
            Option<RefMut<VoxelPillar>>,
            Option<RefMut<VoxelPillar>>,
            Option<RefMut<VoxelPillar>>,
            Option<RefMut<VoxelPillar>>,
        ),
    {
        for (x3, z3) in iproduct!(0..L3DIMENSION, 0..L3DIMENSION) {
            let l2 = &self.data[x3][z3];
            for (x2, z2) in iproduct!(0..L2DIMENSION, 0..L2DIMENSION) {
                let l1 = &l2.data[x2][z2];
                for (x1, z1) in iproduct!(0..L1DIMENSION, 0..L1DIMENSION) {
                    let center = l1.data[x1][z1].borrow_mut();

                    l1.data.split_at_mut(mid)

                    f(
                        center,
                        left_or_down(x3, x2, x1).map(|(sx3, sx2, sx1)| {
                            self.data[sx3][z3].data[sx2][z2].data[sx1][z1].borrow_mut()
                        }),
                        right_or_up(z3, z2, z1).map(|(sz3, sz2, sz1)| {
                            self.data[x3][sz3].data[x2][sz2].data[x1][sz1].borrow_mut()
                        }),
                        right_or_up(x3, x2, x1).map(|(sx3, sx2, sx1)| {
                            self.data[sx3][z3].data[sx2][z2].data[sx1][z1].borrow_mut()
                        }),
                        left_or_down(z3, z2, z1).map(|(sz3, sz2, sz1)| {
                            self.data[x3][sz3].data[x2][sz2].data[x1][sz1].borrow_mut()
                        }),
                    );
                }
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
