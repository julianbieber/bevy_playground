use std::ops::{AddAssign, SubAssign};

use ahash::AHashSet;
use smallvec::SmallVec;

use crate::{
    voxel::{Voxel, VoxelDirection},
    DefaultWorldSector
};

pub trait WaterSimulation {
    fn flow_water(&mut self) -> AHashSet<usize>;
}

/// Represents how much water should flow to the specified index, the maximum length is 4, when water should flow to every direction on the xz plane.
type FlowAmount = SmallVec<[((usize, usize), f32); 4]>;

impl WaterSimulation
    for DefaultWorldSector
{
    fn flow_water(&mut self) -> AHashSet<usize> {
      /*  let directions = [
            VoxelDirection::BACK,
            VoxelDirection::FRONT,
            VoxelDirection::LEFT,
            VoxelDirection::RIGHT,
        ];
        let mut changes = AHashSet::new();
        let mut updates_this_frame = 0;
        for chunk_index in 0..self.data.len() {
            let chunk = &mut self.data[chunk_index];
            if !chunk.data.is_empty() {
                if chunk.meta_data.1 > self.meta_data.0 - 10 {
                    updates_this_frame += 1;
                    chunk.meta_data.1 = 0;
                    for voxel_index in 0..chunk.data.len() {
                        let water_destinations: FlowAmount = match self
                            .data[chunk_index]
                            .get_index(voxel_index)
                        {
                            Voxel::LandVoxel { .. } => SmallVec::new(),
                            Voxel::WaterVoxel { fill, .. } => {
                                let down_flow = self.get_down_flow(*fill, chunk_index, voxel_index);
                                if down_flow.is_empty() {
                                    let surrounding: SmallVec<[((usize, usize), f32); 4]> =
                                        directions
                                            .iter()
                                            .flat_map(|d| {
                                                if let Some((c, v)) = self.index_in_direction(
                                                    *d,
                                                    chunk_index,
                                                    voxel_index,
                                                ) {
                                                    if let Voxel::WaterVoxel { fill: f, .. } =
                                                        self.chunks[c].get_index(v)
                                                    {
                                                        Some(((c, v), *f))
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            })
                                            .filter(|(_, v)| v < fill)
                                            .collect();
                                    let surrounding_len = surrounding.len() as f32;
                                    surrounding
                                        .into_iter()
                                        .map(|(t, f)| (t, (fill - f) / surrounding_len))
                                        .collect()
                                } else {
                                    down_flow
                                }
                            }
                            Voxel::Nothing => SmallVec::new(),
                        };
                        if !water_destinations.is_empty() {
                            changes.insert(chunk_index << 32 & voxel_index);
                        }
                        for ((c, v), amount) in water_destinations {
                            changes.insert(c << 32 & v);
                            match self.data[chunk_index].get_mut_index(voxel_index) {
                                Voxel::LandVoxel { .. } => (),
                                Voxel::WaterVoxel { fill, .. } => {
                                    fill.sub_assign(amount);
                                }
                                Voxel::Nothing => (),
                            }
                            match self.data[c].get_mut_index(v) {
                                Voxel::LandVoxel { .. } => (),
                                Voxel::WaterVoxel { fill, .. } => {
                                    fill.add_assign(amount);
                                }
                                d => {
                                    *d = Voxel::WaterVoxel { fill: amount };
                                }
                            }
                        }
                    }
                } else {
                    chunk.update_age += 1;
                    if chunk.update_age > self.max_update_age {
                        self.max_update_age = chunk.update_age;
                    }
                }
            }

            if updates_this_frame > 100 {
                break;
            }
        }

        changes*/
        AHashSet::new()
    }

    /*fn get_down_flow(&self, fill:f32, chunk: usize, voxel: usize) -> FlowAmount {
        let c = &self.data[chunk];
        let (down_voxel, same_chunk) = c.get_neighbor_index(voxel, VoxelDirection::DOWN);
        let (v, down_chunk_index) = if same_chunk {
            (c.get_index(down_voxel), chunk)
        } else {
            let (down_chunk, same_chunk) = self.get_neighbor_index(chunk, VoxelDirection::DOWN);
            if same_chunk {
                return FlowAmount::new();
            }
            (self.data[down_chunk].get_index(down_voxel), down_chunk)
        };
        
        match v {
            Voxel::LandVoxel { .. } => SmallVec::new(),
            Voxel::WaterVoxel {
                fill: below_fill, ..
            } => {
                let down_flow_amount = (1.0 - below_fill).min(fill);
                if down_flow_amount > 0.001 {
                    smallvec![((down_chunk_index, down_voxel), down_flow_amount)]
                } else {
                    SmallVec::new()
                }
            }
            Voxel::Nothing => smallvec![((down_chunk_index, down_voxel), fill)],
        }
    }*/
}
