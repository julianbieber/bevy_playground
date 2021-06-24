use std::{
    collections::VecDeque,
    ops::{AddAssign, SubAssign},
};

use ahash::{AHashMap, AHashSet};

use crate::voxel_world::{
    access::VoxelAccess,
    voxel::{VoxelDirection, VoxelPosition},
};
use strum::IntoEnumIterator;
pub(super) const WATER_QUADS: usize = 4096 * 40;

#[derive(Debug)]
pub struct Water {
    pub(super) voxels: AHashMap<VoxelPosition, WaterVoxel>,
    pub(super) changed: AHashMap<VoxelPosition, f32>,
    pub(super) unused: VecDeque<[u32; 4]>,
}

/*
Fixed index buffer builds quads from all vertices.
When adding/removing values, only Vertex positions and normals have to be updated.
 */
#[derive(Debug)]
pub(super) struct WaterVoxel {
    pub(super) indices: Vec<[u32; 4]>,
    pub(super) fill: f32,
}

impl Water {
    pub fn new() -> Water {
        let mut unused = VecDeque::with_capacity(WATER_QUADS);
        for i in 0..WATER_QUADS {
            unused.push_back([
                i as u32 * 4 + 0,
                i as u32 * 4 + 1,
                i as u32 * 4 + 2,
                i as u32 * 4 + 3,
            ]);
        }
        Water {
            voxels: AHashMap::new(),
            changed: AHashMap::new(),
            unused,
        }
    }

    pub fn flow(&mut self, voxel_access: &VoxelAccess) {
        let directions = [
            VoxelDirection::BACK,
            VoxelDirection::FRONT,
            VoxelDirection::LEFT,
            VoxelDirection::RIGHT,
        ];
        for (position, water) in self.voxels.iter() {
            if let Some(below) = self
                .voxels
                .get(&position.in_direction(VoxelDirection::DOWN))
            {
                let down_flow_amount = (1.0 - below.fill
                    + self
                        .changed
                        .get(&&position.in_direction(VoxelDirection::DOWN))
                        .map(|v| v.clone())
                        .unwrap_or(0.0))
                .min(water.fill);
                if down_flow_amount > 0.001 {
                    self.changed
                        .entry(position.in_direction(VoxelDirection::DOWN))
                        .or_insert(0.0)
                        .add_assign(down_flow_amount);
                    self.changed
                        .entry(*position)
                        .or_insert(0.0)
                        .sub_assign(down_flow_amount);
                    continue;
                }
            } else {
                if voxel_access
                    .get_voxel(position.in_direction(VoxelDirection::DOWN))
                    .is_none()
                {
                    let down_flow_amount = water.fill.min(
                        (1.0 - self
                            .changed
                            .get(&&position.in_direction(VoxelDirection::DOWN))
                            .map(|v| v.clone())
                            .unwrap_or(0.0))
                        .min(water.fill),
                    );
                    self.changed
                        .entry(position.in_direction(VoxelDirection::DOWN))
                        .or_insert(0.0)
                        .add_assign(down_flow_amount);
                    self.changed
                        .entry(*position)
                        .or_insert(0.0)
                        .sub_assign(down_flow_amount);
                    continue;
                }
            }
            if water.fill > 0.1 {
                let surrounding: Vec<(VoxelPosition, f32)> = {
                    directions
                        .iter()
                        .map(|d| {
                            let other_position = position.in_direction(*d);
                            (
                                other_position,
                                self.voxels
                                    .get(&other_position)
                                    .map(|v| v.fill)
                                    .unwrap_or(0.0),
                            )
                        })
                        .filter(|(_, v)| *v < water.fill)
                        .filter(|(p, _)| voxel_access.get_voxel(*p).is_none())
                        .collect()
                };

                let surrounding_len = (surrounding.len() + 1) as f32;
                let mut remaining = water.fill;
                for (surrounding_position, surrounding_water) in surrounding {
                    let flow_amount = ((water.fill - surrounding_water) / surrounding_len)
                        .min(remaining)
                        .min(
                            1.0 - surrounding_water
                                + self
                                    .changed
                                    .get(&surrounding_position)
                                    .map(|v| v.clone())
                                    .unwrap_or(0.0),
                        );
                    remaining -= flow_amount;
                    self.changed
                        .entry(surrounding_position)
                        .or_insert(0.0)
                        .add_assign(flow_amount);
                    self.changed
                        .entry(*position)
                        .or_insert(0.0)
                        .sub_assign(flow_amount);
                }
            }
        }
    }

    /// returns a set of VoxelPositions, indicating which positions should be remeshed
    pub fn apply_changes(&mut self) -> AHashSet<VoxelPosition> {
        let mut changed = AHashSet::new();
        for (position, amount) in self.changed.iter() {
            if let Some(water) = self.voxels.get_mut(position) {
                water.fill.add_assign(amount);
            } else {
                if *amount > 0.0 {
                    self.voxels.insert(
                        *position,
                        WaterVoxel {
                            fill: *amount,
                            indices: vec![],
                        },
                    );
                }
            }
            changed.insert(*position);
            for d in VoxelDirection::iter() {
                changed.insert(position.in_direction(d));
            }
        }

        for (_, v) in self.voxels.iter_mut() {
            if v.fill > 0.9 {
                v.fill = 1.0;
            }
            if v.fill < 0.0 {
                v.fill = 0.0;
            }
        }

        //  TODO fix overfill and underfill

        self.changed = AHashMap::new();

        changed
    }
}
