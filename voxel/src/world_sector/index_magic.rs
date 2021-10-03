use crate::voxel::{Voxel, VoxelDirection};

use super::{WorldChunk, WorldSector};

impl<const CHUNKS_LOADED: i32, const CHUNK_SIZE: i32> WorldSector<CHUNKS_LOADED, CHUNK_SIZE> {
    pub fn index_in_direction(
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
}

pub fn surrounding<const SIZE: usize, const I: usize>(
    i: usize,
    directions: [VoxelDirection; I],
) -> [(VoxelDirection, Option<usize>); I] {
    directions.map(|d| {
        let index = match d {
            VoxelDirection::UP => {
                if i % SIZE.pow(2) >= SIZE.pow(2) - SIZE {
                    None
                } else {
                    Some(i + SIZE)
                }
            }
            VoxelDirection::DOWN => {
                if (i % SIZE.pow(2)) < SIZE {
                    None
                } else {
                    Some(i - SIZE as usize)
                }
            }
            VoxelDirection::LEFT => {
                if i % SIZE == 0 {
                    None
                } else {
                    Some(i - 1)
                }
            }
            VoxelDirection::RIGHT => {
                if i % SIZE == SIZE - 1 {
                    None
                } else {
                    Some(i + 1)
                }
            }
            VoxelDirection::FRONT => {
                if i < SIZE.pow(2) {
                    None
                } else {
                    Some(i - SIZE.pow(2))
                }
            }
            VoxelDirection::BACK => {
                if i >= (SIZE.pow(3) - SIZE.pow(2)) {
                    None
                } else {
                    Some(i + SIZE.pow(2))
                }
            }
        };
        (d, index)
    })
}

pub fn indices_at_boundary<const SIZE: usize, const SQUARE: usize>(
    direction: VoxelDirection,
) -> [usize; SQUARE] {
    let mut values = [0; SQUARE];
    match direction {
        VoxelDirection::UP => {
            for i in 0..SQUARE {
                values[i] = SQUARE - SIZE + i % SIZE + (i / SIZE * SQUARE)
            }
        }
        VoxelDirection::DOWN => {
            for i in 0..SQUARE {
                values[i] = i % SIZE + (i / SIZE) * SQUARE
            }
        }
        VoxelDirection::LEFT => {
            for i in 0..SQUARE {
                values[i] = i * SIZE;
            }
        }
        VoxelDirection::RIGHT => {
            for i in 0..SQUARE {
                values[i] = i * SIZE + SIZE - 1;
            }
        }
        VoxelDirection::FRONT => {
            for i in 0..SQUARE {
                values[i] = i
            }
        }
        VoxelDirection::BACK => {
            for i in 0..SQUARE {
                values[i] = i + SIZE.pow(3) - SQUARE
            }
        }
    }
    values
}
