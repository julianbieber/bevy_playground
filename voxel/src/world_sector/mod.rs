mod index_magic;
pub mod water_simulation;

use crate::voxel::{Voxel, VoxelDirection};
use crate::world_gen::Generator;
use crate::{boundaries::ChunkBoundaries, voxel::VoxelPosition};
use ahash::AHashSet;
use index_magic::indices_at_boundary;

use self::index_magic::surrounding;

const DEFAULT_LOADED_CHUNKS: i32 = 128;
const DEFAULT_LOADED_CHUNKS_USIZE: usize = DEFAULT_LOADED_CHUNKS as usize;
const DEFAULT_LOADED_CHUNKS_USIZE_SQ: usize = (DEFAULT_LOADED_CHUNKS as usize).pow(2);

const DEFAULT_CHUNK_SIZE: i32 = 8;
const DEFAULT_CHUNK_SIZE_USIZE: usize = DEFAULT_CHUNK_SIZE as usize;
const DEFAULT_CHUNK_SIZE_USIZE_SQ: usize = (DEFAULT_CHUNK_SIZE as usize).pow(2);

pub type DefaultWorldSector = WorldSector<DEFAULT_LOADED_CHUNKS, DEFAULT_CHUNK_SIZE>;

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
        dbg!(self.chunks.len(), CHUNKS_LOADED);
    }

    pub fn insert_terrain(&mut self) {
        let mut chunks_initialized: AHashSet<usize> = AHashSet::new();
        let mut chunks_simplified: AHashSet<usize> = AHashSet::new();
        let mut generated_before_simplified = 0;
        for chunk_index in indices_at_boundary::<
            DEFAULT_LOADED_CHUNKS_USIZE,
            DEFAULT_LOADED_CHUNKS_USIZE_SQ,
        >(VoxelDirection::DOWN)
        {
            for voxel_index in indices_at_boundary::<
                DEFAULT_CHUNK_SIZE_USIZE,
                DEFAULT_CHUNK_SIZE_USIZE_SQ,
            >(VoxelDirection::DOWN)
            {
                let ground_position = &self.chunks[chunk_index].index_to_coord(voxel_index);
                let ys = self.generator.generate_chunk::<512>(
                    ground_position.x,
                    ground_position.y..ground_position.y + (CHUNKS_LOADED * CHUNK_SIZE),
                    ground_position.z,
                );
                let mut current_chunk_index = chunk_index;
                let mut current_voxel_index = voxel_index;

                for typ in ys {
                    *self.chunks[current_chunk_index].get_mut_index(current_voxel_index) =
                        Voxel::LandVoxel { typ };
                    chunks_initialized.insert(current_chunk_index);
                    if let Some(updated) = self.index_in_direction(
                        VoxelDirection::UP,
                        current_chunk_index,
                        current_voxel_index,
                    ) {
                        current_chunk_index = updated.0;
                        current_voxel_index = updated.1;
                    }
                }
            }

            if generated_before_simplified % (DEFAULT_LOADED_CHUNKS_USIZE_SQ / 16)
                == DEFAULT_LOADED_CHUNKS_USIZE_SQ / 16 - 1
            { // The idea is to keep track of which chunks have already been simplified or can not be simplified even after further chunks have been calculated
                for &c in chunks_initialized.iter() {
                    if !chunks_simplified.contains(&c) {
                        let mut surrounding_filled = 0;
                        for (d, surrounding_chunk_o) in surrounding::<DEFAULT_LOADED_CHUNKS_USIZE, 6>(
                            c,
                            [
                                VoxelDirection::UP,
                                VoxelDirection::DOWN,
                                VoxelDirection::LEFT,
                                VoxelDirection::RIGHT,
                                VoxelDirection::FRONT,
                                VoxelDirection::BACK,
                            ],
                        ) {
                            let side_filled = if let Some(surrounding_chunk) = surrounding_chunk_o {
                                if !chunks_initialized.contains(&surrounding_chunk) {
                                    break;
                                }

                                let (other_direction, my_direction) = match d {
                                    VoxelDirection::UP => (VoxelDirection::DOWN, d),
                                    VoxelDirection::DOWN => (VoxelDirection::UP, d),
                                    VoxelDirection::LEFT => (VoxelDirection::RIGHT, d),
                                    VoxelDirection::RIGHT => (VoxelDirection::LEFT, d),
                                    VoxelDirection::FRONT => (VoxelDirection::BACK, d),
                                    VoxelDirection::BACK => (VoxelDirection::FRONT, d),
                                };

                                self.count_land_voxels::<DEFAULT_CHUNK_SIZE_USIZE_SQ>(
                                    &self.chunks[surrounding_chunk],
                                    &indices_at_boundary::<
                                        DEFAULT_CHUNK_SIZE_USIZE,
                                        DEFAULT_CHUNK_SIZE_USIZE_SQ,
                                    >(other_direction),
                                ) == DEFAULT_CHUNK_SIZE_USIZE_SQ
                                    && self.count_land_voxels::<DEFAULT_CHUNK_SIZE_USIZE_SQ>(
                                        &self.chunks[c],
                                        &indices_at_boundary::<
                                            DEFAULT_CHUNK_SIZE_USIZE,
                                            DEFAULT_CHUNK_SIZE_USIZE_SQ,
                                        >(my_direction),
                                    ) == DEFAULT_CHUNK_SIZE_USIZE_SQ
                            } else {
                                true
                            };
                            if side_filled
                                || surrounding_chunk_o
                                    .map(|surrounding_chunk| {
                                        chunks_simplified.contains(&surrounding_chunk)
                                    })
                                    .unwrap_or(true)
                            {
                                surrounding_filled += 1;
                            }
                        }
                        if surrounding_filled == 6 {
                            chunks_simplified.insert(c);
                            let chunk = &mut self.chunks[c];
                            chunk.reduce_storage();
                        } else {
                        }
                    }
                }
            }

            generated_before_simplified += 1;
        }
    }

    fn count_land_voxels<const I: usize>(
        &self,
        chunk: &WorldChunk<CHUNK_SIZE>,
        indices: &[usize; I],
    ) -> usize {
        let mut existing_voxels = 0;
        for &other_index in indices {
            match chunk.get_index(other_index) {
                Voxel::LandVoxel { .. } => existing_voxels += 1,
                Voxel::WaterVoxel { .. } => (),
                Voxel::Nothing => (),
            }
        }
        existing_voxels
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

    pub fn get_surrounding<const I: usize>(
        &self,
        center_chunk: usize,
        center_voxel: usize,
        directions: [VoxelDirection; I],
    ) -> [(VoxelDirection, Option<Voxel>); I] {
        directions.map(|d| {
            (
                d,
                self.index_in_direction(d, center_chunk, center_voxel)
                    .map(|(c, v)| self.chunks[c].get_index(v).clone()),
            )
        })
    }

    pub(super) fn index_from_min(&self, min: VoxelPosition) -> usize {
        let offseted = (min - self.chunks[0].boundaries.min) / CHUNK_SIZE;
        (offseted.z * CHUNKS_LOADED * CHUNKS_LOADED + offseted.y * CHUNKS_LOADED + offseted.x)
            as usize
    }

    fn index(&self, min: VoxelPosition, position: VoxelPosition) -> usize {
        let offseted = position - min;
        (offseted.z * CHUNK_SIZE.pow(2) + offseted.y * CHUNK_SIZE + offseted.x) as usize
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

    fn reduce_storage(&mut self) {
        self.voxels = vec![];
        self.default = Voxel::LandVoxel {
            typ: crate::voxel::VoxelTypes::Moss,
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
        let offseted = position - self.boundaries.min;
        (offseted.z * SIZE * SIZE + offseted.y * SIZE + offseted.x) as usize
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
    fn walk_through_sector_front() {
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
                    sector.index_in_direction(direction, current_chunk_i, current_voxel_i)
                {
                    current_chunk_i = ci;
                    current_voxel_i = vi;
                } else {
                    dbg!(direction);
                    assert_eq!(true, false);
                }
            }

            assert_eq!(
                sector.index_in_direction(direction, current_chunk_i, current_voxel_i),
                None
            );
        }
        assert_eq!(current_chunk_i, 8 * 8 * 8 - 1);
        assert_eq!(current_voxel_i, 8 * 8 * 8 - 1);
    }

    #[test]
    fn walk_through_sector_back() {
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
                    sector.index_in_direction(direction, current_chunk_i, current_voxel_i)
                {
                    current_chunk_i = ci;
                    current_voxel_i = vi;
                } else {
                    dbg!(direction);
                    assert_eq!(true, false);
                }
            }

            assert_eq!(
                sector.index_in_direction(direction, current_chunk_i, current_voxel_i),
                None
            );
        }
        assert_eq!(current_chunk_i, 0);
        assert_eq!(current_voxel_i, 0);
    }
}
