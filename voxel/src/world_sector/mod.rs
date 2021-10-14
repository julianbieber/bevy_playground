mod index_magic;
pub mod water_simulation;
mod generic_chunk;
mod iteration;

use crate::voxel::{Voxel, VoxelDirection, VoxelTypes};
use crate::world_gen::Generator;
use crate::{boundaries::ChunkBoundaries, voxel::VoxelPosition};
use ahash::AHashSet;

use self::generic_chunk::MetaChunk;

const DEFAULT_LOADED_CHUNKS: i32 = 128;
const DEFAULT_LOADED_CHUNKS_USIZE: usize = DEFAULT_LOADED_CHUNKS as usize;
const DEFAULT_LOADED_CHUNKS_USIZE_SQ: usize = (DEFAULT_LOADED_CHUNKS as usize).pow(2);

const DEFAULT_CHUNK_SIZE: i32 = 8;
const DEFAULT_CHUNK_SIZE_USIZE: usize = DEFAULT_CHUNK_SIZE as usize;
const DEFAULT_CHUNK_SIZE_USIZE_SQ: usize = (DEFAULT_CHUNK_SIZE as usize).pow(2);

type VoxelChunk = MetaChunk<Voxel, (Voxel, i32), DEFAULT_CHUNK_SIZE_USIZE, DEFAULT_CHUNK_SIZE, DEFAULT_CHUNK_SIZE_USIZE_SQ>; 

pub type DefaultWorldSector = MetaChunk<
    VoxelChunk,
    (i32, Generator), 
    DEFAULT_LOADED_CHUNKS_USIZE, 
    DEFAULT_LOADED_CHUNKS, 
    DEFAULT_CHUNK_SIZE_USIZE_SQ
>;

impl DefaultWorldSector {
    pub fn new(center: VoxelPosition) -> DefaultWorldSector {
        let center_chunk_min = ChunkBoundaries::<DEFAULT_CHUNK_SIZE>::aligned(center);
        let min = center_chunk_min.min - VoxelPosition::diagonal((DEFAULT_LOADED_CHUNKS / 2) * DEFAULT_CHUNK_SIZE);
        let mut chunks = DefaultWorldSector {
            data: Vec::new(),
            min,
            meta_data: (0, Generator::new()),
        };
        chunks.init(min);
        chunks
    }

    fn init(&mut self, min: VoxelPosition) {
        self.data.reserve(DEFAULT_LOADED_CHUNKS_USIZE.pow(3));
        for i in 0..DEFAULT_LOADED_CHUNKS_USIZE.pow(3) {
            self.data.push(VoxelChunk::empty(self.min_coordinates_from_index(i) + min, Voxel::Nothing));
        }
    }

    pub fn insert_terrain(&mut self) {
        let mut chunks_initialized: AHashSet<usize> = AHashSet::new();
        let mut chunks_simplified: AHashSet<usize> = AHashSet::new();
        let mut generated_before_simplified = 0;
        let chunks_boundary = self.indices_at_boundary(VoxelDirection::DOWN);
        for chunk_index in chunks_boundary
        {
            let c = &self.data[chunk_index];
            for voxel_index in c.indices_at_boundary(VoxelDirection::DOWN)
            {
                let ground_position = c.index_to_coord(voxel_index);
                let ys = self.meta_data.1.generate_chunk::<512>(
                    ground_position.x,
                    ground_position.y..ground_position.y + (DEFAULT_LOADED_CHUNKS * DEFAULT_CHUNK_SIZE),
                    ground_position.z,
                );
                let mut current_chunk_index = chunk_index;
                let mut current_voxel_index = voxel_index;

                for typ in ys {
                    *self.data[current_chunk_index].get_mut_index(current_voxel_index) =
                        Voxel::LandVoxel { typ };
                    chunks_initialized.insert(current_chunk_index);
                    let (new_voxel_index, same_chunk) = self.data[current_chunk_index].get_neighbor_index(current_voxel_index, VoxelDirection::UP);
                    current_voxel_index = new_voxel_index;
                    if !same_chunk {
                        let (new_chunk_index, same_chunk) = self.get_neighbor_index(current_chunk_index, VoxelDirection::UP);
                        if !same_chunk {
                            panic!("tried to move out of loaded chunks while initializing the world in the up direction")
                        }
                        current_chunk_index = new_chunk_index;
                    }
                }
            }

            if generated_before_simplified % (DEFAULT_LOADED_CHUNKS_USIZE_SQ / 16)
                == DEFAULT_LOADED_CHUNKS_USIZE_SQ / 16 - 1
            { // The idea is to keep track of which chunks have already been simplified or can not be simplified even after further chunks have been calculated
                for &c in chunks_initialized.iter() {
                    if !chunks_simplified.contains(&c) {
                        let mut surrounding_filled = 0;
                        for (d, surrounding_chunk_o) in self.surrounding(
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
                            let side_filled = if let (surrounding_chunk, true) = surrounding_chunk_o {
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

                                self.data[surrounding_chunk].count_outside_land_voxels(other_direction) == DEFAULT_CHUNK_SIZE_USIZE_SQ
                                    && self.data[c].count_outside_land_voxels(my_direction) == DEFAULT_CHUNK_SIZE_USIZE_SQ
                            } else {
                                true
                            };
                            if side_filled
                                || !surrounding_chunk_o.1
                                || chunks_simplified.contains(&surrounding_chunk_o.0)
                            {
                                surrounding_filled += 1;
                            }
                        }
                        if surrounding_filled == 6 {
                            chunks_simplified.insert(c);
                            let chunk = &mut self.data[c];
                            chunk.reduce_storage(Voxel::LandVoxel{typ: VoxelTypes::Moss});
                        } else {
                        }
                    }
                }
            }

            generated_before_simplified += 1;
        }
    }

    fn min_coordinates_from_index(&self, index: usize) -> VoxelPosition {
        let z = index as i32 / (DEFAULT_LOADED_CHUNKS.pow(2)) * DEFAULT_CHUNK_SIZE;
        let y = (index as i32 / DEFAULT_LOADED_CHUNKS) % DEFAULT_LOADED_CHUNKS * DEFAULT_CHUNK_SIZE;
        let x = index as i32 % DEFAULT_LOADED_CHUNKS * DEFAULT_CHUNK_SIZE;
        VoxelPosition { x, y, z }
    }

    pub fn insert(&mut self, position: VoxelPosition, voxel: Voxel) {
        let min = ChunkBoundaries::<DEFAULT_CHUNK_SIZE>::aligned(position).min;
        let chunk_index = self.index_from_min(min);
        let mut c = self.data[chunk_index];
        let i = c.local_index(position);
        *c.get_mut_index(i) = voxel;
    }

    pub(super) fn index_from_min(&self, min: VoxelPosition) -> usize {
        let offseted = (min - self.min) / DEFAULT_CHUNK_SIZE;
        (offseted.z * DEFAULT_LOADED_CHUNKS * DEFAULT_LOADED_CHUNKS + offseted.y * DEFAULT_LOADED_CHUNKS + offseted.x)
            as usize
    }
}
