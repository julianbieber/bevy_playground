use std::ops::{AddAssign, SubAssign};

use smallvec::SmallVec;

use crate::{
    voxel::{Voxel, VoxelDirection},
    world_sector::WorldSector,
};

pub trait WaterSimulation<const CHUNKS_LOADED: i32, const CHUNK_SIZE: i32> {
    fn flow_water(&mut self);
}

impl<const CHUNKS_LOADED: i32, const CHUNK_SIZE: i32> WaterSimulation<CHUNKS_LOADED, CHUNK_SIZE>
    for WorldSector<CHUNKS_LOADED, CHUNK_SIZE>
{
    fn flow_water(&mut self) {
        let directions = [
            VoxelDirection::BACK,
            VoxelDirection::FRONT,
            VoxelDirection::LEFT,
            VoxelDirection::RIGHT,
        ];
        let mut updates_this_frame = 0;
        for chunk_index in 0..self.chunks.len() {
            let chunk = &mut self.chunks[chunk_index];
            if !chunk.water.is_empty() {
                if chunk.update_age > self.max_update_age - 10 {
                    updates_this_frame += 1;
                    chunk.update_age = 0;
                    for voxel_index in 0..CHUNK_SIZE.pow(3) as usize {
                        let water_destinations: SmallVec<[((usize, usize), f32); 4]> = match self
                            .chunks[chunk_index]
                            .get_index(voxel_index)
                        {
                            Voxel::LandVoxel { .. } => SmallVec::new(),
                            Voxel::WaterVoxel { fill, .. } => {
                                let down_flow = if let Some((below_c, below_v)) = self
                                    .index_in_directorion(
                                        VoxelDirection::DOWN,
                                        chunk_index,
                                        voxel_index,
                                    ) {
                                    match self.chunks[below_c].get_index(below_v) {
                                        Voxel::LandVoxel { .. } => SmallVec::new(),
                                        Voxel::WaterVoxel {
                                            fill: below_fill, ..
                                        } => {
                                            let down_flow_amount = (1.0 - below_fill).min(*fill);
                                            if down_flow_amount > 0.001 {
                                                smallvec![((below_c, below_v), down_flow_amount)]
                                            } else {
                                                SmallVec::new()
                                            }
                                        }
                                        Voxel::Nothing => smallvec![((below_c, below_v), *fill)],
                                    }
                                } else {
                                    SmallVec::new()
                                };
                                if down_flow.is_empty() {
                                    let surrounding: SmallVec<[((usize, usize), f32); 4]> =
                                        directions
                                            .iter()
                                            .flat_map(|d| {
                                                if let Some((c, v)) = self.index_in_directorion(
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

                        for ((c, v), amount) in water_destinations {
                            match self.chunks[chunk_index].get_mut_index(voxel_index) {
                                Voxel::LandVoxel { .. } => (),
                                Voxel::WaterVoxel { fill, .. } => {
                                    fill.sub_assign(amount);
                                }
                                Voxel::Nothing => (),
                            }
                            match self.chunks[c].get_mut_index(v) {
                                Voxel::LandVoxel { .. } => (),
                                Voxel::WaterVoxel { fill, .. } => {
                                    fill.add_assign(amount);
                                }
                                d => {
                                    *d = Voxel::WaterVoxel {
                                        fill: amount,
                                        used_indices: vec![],
                                    };
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
    }
}
