use std::borrow::Borrow;

use crate::generator;
use crate::voxel::{Voxel, VoxelDirection};
use crate::world_gen::Generator;
use crate::{boundaries::ChunkBoundaries, voxel::VoxelPosition};

const LOADED_WATER_CHUNKS: i32 = 8;

const WATER_CHUNK_SIZE: i32 = 8;

pub type DefaultWorldSector = WorldSector<LOADED_WATER_CHUNKS, WATER_CHUNK_SIZE>;

/// Water chunks represents the currently loaded cube of chunks in the players vicinity.
/// The chunk at position 0 has a min value corresponding to the min in WaterChunks
pub struct WorldSector<const CHUNKS_LOADED: i32, const CHUNK_SIZE: i32> {
    pub(super) chunks: Vec<WorldChunk<CHUNK_SIZE>>,
    pub(super) max_update_age: i32,
    generator: Generator,
}

impl<const CHUNKS_LOADED: i32, const CHUNK_SIZE: i32> WorldSector<CHUNKS_LOADED, CHUNK_SIZE> {
    pub fn new(center: VoxelPosition) -> WorldSector<CHUNKS_LOADED, CHUNK_SIZE> {
        let center_chunk_min = ChunkBoundaries::<CHUNK_SIZE>::aligned(center);
        let min = center_chunk_min.min - VoxelPosition::diagonal((CHUNKS_LOADED / 2) * CHUNK_SIZE);
        let mut chunks = WorldSector {
            chunks: Vec::new(),
            max_update_age: 0,
            generator: Generator::new(),
        };
        chunks.init(min);
        chunks
    }

    fn init(&mut self, min: VoxelPosition) {
        self.chunks.reserve(CHUNKS_LOADED.pow(3) as usize);
        for i in 0..CHUNKS_LOADED.pow(3) as usize {
            self.chunks
                .push(WorldChunk::empty(self.min_coordinates_from_index(i) + min));
        }
    }

    pub fn generate_world(&mut self) {
        let min = self.chunks[0].boundaries.min;
        for x in min.x..min.x + (CHUNKS_LOADED * CHUNK_SIZE) {
            for z in min.z..min.z + (CHUNKS_LOADED * CHUNK_SIZE) {
                let ground = VoxelPosition::new(x, min.y, z);
                let ground_min = ChunkBoundaries::<CHUNK_SIZE>::aligned(ground).min;
                let mut ground_chunk_index = self.index_from_min(ground_min);
                let mut y_i = self.chunks[ground_chunk_index].index(ground);
                let ys = self.generator.generate_chunk(
                    x,
                    min.y..min.y + (CHUNKS_LOADED * CHUNK_SIZE),
                    z,
                );
                for typ in ys {
                    *self.chunks[ground_chunk_index].get_mut_index(y_i) = Voxel::LandVoxel { typ };

                    if let Some(updated) =
                        self.index_in_directorion(VoxelDirection::UP, ground_chunk_index, y_i)
                    {
                        ground_chunk_index = updated.0;
                        y_i = updated.1;
                    }
                }
            }
        }

        for chunk in self.chunks.iter_mut() {
            chunk.reduceStorage();
        }
    }

    fn min_coordinates_from_index(&self, index: usize) -> VoxelPosition {
        let z = index as i32 / (CHUNKS_LOADED * CHUNKS_LOADED) * CHUNK_SIZE;
        let y = (index as i32 / CHUNKS_LOADED) % CHUNKS_LOADED * CHUNK_SIZE;
        let x = index as i32 % CHUNKS_LOADED * CHUNK_SIZE;
        VoxelPosition { x, y, z }
    }

    pub fn insert(&mut self, position: VoxelPosition, voxel: Voxel) {
        let min = ChunkBoundaries::<CHUNK_SIZE>::aligned(position).min;
        let chunk_index = self.index_from_min(min);
        let i = self.index(min, position);
        *self.chunks[chunk_index].get_mut_index(i) = voxel;
    }

    pub(super) fn index_in_directorion(
        &self,
        direction: VoxelDirection,
        chunk: usize,
        voxel: usize,
    ) -> Option<(usize, usize)> {
        match direction {
            VoxelDirection::UP => {
                if (voxel) % CHUNK_SIZE.pow(2) as usize >= (CHUNK_SIZE.pow(2) - CHUNK_SIZE) as usize
                {
                    if chunk % CHUNKS_LOADED.pow(2) as usize
                        >= (CHUNKS_LOADED.pow(2) - CHUNKS_LOADED) as usize
                    {
                        None
                    } else {
                        Some((
                            chunk + CHUNKS_LOADED as usize,
                            voxel - (CHUNK_SIZE.pow(2) as usize - CHUNK_SIZE as usize),
                        ))
                    }
                } else {
                    Some((chunk, voxel + CHUNK_SIZE as usize))
                }
            }
            VoxelDirection::DOWN => {
                if (voxel % CHUNK_SIZE.pow(2) as usize) < (CHUNK_SIZE as usize) {
                    if (chunk % CHUNKS_LOADED.pow(2) as usize) < CHUNKS_LOADED as usize {
                        None
                    } else {
                        Some((
                            chunk - CHUNKS_LOADED as usize,
                            voxel + (CHUNK_SIZE.pow(2) as usize - CHUNK_SIZE as usize),
                        ))
                    }
                } else {
                    Some((chunk, voxel - CHUNK_SIZE as usize))
                }
            }
            VoxelDirection::LEFT => {
                if voxel % CHUNK_SIZE as usize == 0 {
                    if chunk % CHUNKS_LOADED as usize == 0 {
                        None
                    } else {
                        Some((chunk - 1, voxel + (CHUNK_SIZE as usize - 1)))
                    }
                } else {
                    Some((chunk, voxel - 1))
                }
            }
            VoxelDirection::RIGHT => {
                if voxel % CHUNK_SIZE as usize == CHUNK_SIZE as usize - 1 {
                    if chunk % CHUNKS_LOADED as usize == CHUNKS_LOADED as usize - 1 {
                        None
                    } else {
                        Some((chunk + 1, voxel - (CHUNK_SIZE as usize - 1)))
                    }
                } else {
                    Some((chunk, voxel + 1))
                }
            }
            VoxelDirection::FRONT => {
                if voxel < CHUNK_SIZE.pow(2) as usize {
                    if chunk < CHUNKS_LOADED.pow(2) as usize {
                        None
                    } else {
                        Some((
                            chunk - CHUNKS_LOADED.pow(2) as usize,
                            voxel + CHUNK_SIZE.pow(3) as usize - CHUNK_SIZE.pow(2) as usize,
                        ))
                    }
                } else {
                    Some((chunk, voxel - CHUNK_SIZE.pow(2) as usize))
                }
            }
            VoxelDirection::BACK => {
                if voxel >= (CHUNK_SIZE.pow(3) - CHUNK_SIZE.pow(2)) as usize {
                    if chunk >= (CHUNKS_LOADED.pow(3) - CHUNKS_LOADED.pow(2)) as usize {
                        None
                    } else {
                        Some((
                            chunk + CHUNKS_LOADED.pow(2) as usize,
                            voxel - (CHUNK_SIZE.pow(3) as usize - CHUNK_SIZE.pow(2) as usize),
                        ))
                    }
                } else {
                    Some((chunk, voxel + CHUNK_SIZE.pow(2) as usize))
                }
            }
        }
    }

    pub(super) fn index_from_min(&self, min: VoxelPosition) -> usize {
        let offsetted = (min - self.chunks[0].boundaries.min) / CHUNK_SIZE;
        (offsetted.z * CHUNKS_LOADED * CHUNKS_LOADED + offsetted.y * CHUNKS_LOADED + offsetted.x)
            as usize
    }

    fn index(&self, min: VoxelPosition, position: VoxelPosition) -> usize {
        let offsetted = position - min;
        (offsetted.z * CHUNK_SIZE.pow(2) + offsetted.y * CHUNK_SIZE + offsetted.x) as usize
    }

    fn in_bounds(&self, coordinate: VoxelPosition) -> bool {
        self.chunks[0].boundaries.min.x <= coordinate.x
            && self.chunks[0].boundaries.min.y <= coordinate.y
            && self.chunks[0].boundaries.min.z <= coordinate.z
            && self.chunks.last().unwrap().boundaries.max.x > coordinate.x
            && self.chunks.last().unwrap().boundaries.max.y > coordinate.y
            && self.chunks.last().unwrap().boundaries.max.z > coordinate.z
    }

    pub fn water_count(&self) -> usize {
        self.chunks.iter().map(|c| c.water_count()).sum()
    }
}

pub(super) struct WorldChunk<const SIZE: i32> {
    pub(super) voxels: Vec<Voxel>,
    pub(super) boundaries: ChunkBoundaries<SIZE>,
    pub(super) default: Voxel,
    pub(super) update_age: i32,
}

impl<const SIZE: i32> WorldChunk<SIZE> {
    fn empty(min: VoxelPosition) -> WorldChunk<SIZE> {
        WorldChunk {
            voxels: vec![],
            boundaries: ChunkBoundaries::<SIZE>::aligned(min),
            default: Voxel::Nothing, // needed to return a ref to the voxel in the get_* methods (as far as I know)
            update_age: 0,
        }
    }

    pub fn reduceStorage(&mut self) {
        if self.voxels.len() == SIZE as usize {
            self.voxels = vec![];
            self.default = Voxel::LandVoxel {
                typ: crate::voxel::VoxelTypes::Moss,
            } // TODO chose representation based on exisiting
        }
    }

    pub(super) fn get_index(&self, i: usize) -> &Voxel {
        if self.voxels.is_empty() {
            &self.default
        } else {
            &self.voxels[i]
        }
    }

    pub(super) fn get_mut_index(&mut self, i: usize) -> &mut Voxel {
        if self.voxels.is_empty() {
            self.voxels = vec![self.default; SIZE.pow(3) as usize]
        }
        &mut self.voxels[i]
    }

    pub(super) fn index(&self, position: VoxelPosition) -> usize {
        let offsetted = position - self.boundaries.min;
        (offsetted.z * SIZE * SIZE + offsetted.y * SIZE + offsetted.x) as usize
    }

    pub(super) fn index_to_coord(&self, i: usize) -> VoxelPosition {
        let z = i as i32 / (SIZE * SIZE);
        let y = (i as i32 / SIZE) % SIZE;
        let x = i as i32 % SIZE;
        VoxelPosition { x, y, z } + self.boundaries.min
    }

    pub fn water_count(&self) -> usize {
        if self.voxels.len() == 0 {
            0
        } else {
            self.voxels
                .iter()
                .map(|v| match v {
                    Voxel::LandVoxel { .. } => 0,
                    Voxel::WaterVoxel { .. } => 1,
                    Voxel::Nothing => 0,
                })
                .sum()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::voxel::VoxelDirection;

    use super::WorldSector;

    #[test]
    fn walk_throguh_sector_front() {
        let sector = WorldSector::<8, 8>::new([0, 0, 0].into());
        let mut current_chunk_i: usize = 0;
        let mut current_voxel_i: usize = 0;
        for direction in [
            VoxelDirection::RIGHT,
            VoxelDirection::UP,
            VoxelDirection::BACK,
        ] {
            for _chunk in 0..63 {
                if let Some((ci, vi)) =
                    sector.index_in_directorion(direction, current_chunk_i, current_voxel_i)
                {
                    current_chunk_i = ci;
                    current_voxel_i = vi;
                } else {
                    dbg!(direction);
                    assert_eq!(true, false);
                }
            }

            assert_eq!(
                sector.index_in_directorion(direction, current_chunk_i, current_voxel_i),
                None
            );
        }
        assert_eq!(current_chunk_i, 8 * 8 * 8 - 1);
        assert_eq!(current_voxel_i, 8 * 8 * 8 - 1);
    }

    #[test]
    fn walk_throguh_sector_back() {
        let sector = WorldSector::<8, 8>::new([0, 0, 0].into());
        let mut current_chunk_i: usize = 8 * 8 * 8 - 1;
        let mut current_voxel_i: usize = 8 * 8 * 8 - 1;
        for direction in [
            VoxelDirection::LEFT,
            VoxelDirection::DOWN,
            VoxelDirection::FRONT,
        ] {
            for _chunk in 0..63 {
                if let Some((ci, vi)) =
                    sector.index_in_directorion(direction, current_chunk_i, current_voxel_i)
                {
                    current_chunk_i = ci;
                    current_voxel_i = vi;
                } else {
                    dbg!(direction);
                    assert_eq!(true, false);
                }
            }

            assert_eq!(
                sector.index_in_directorion(direction, current_chunk_i, current_voxel_i),
                None
            );
        }
        assert_eq!(current_chunk_i, 0);
        assert_eq!(current_voxel_i, 0);
    }
}
