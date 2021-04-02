use crate::voxel_world::voxel::{Voxel, VoxelPosition};

use super::{
    boundaries::{ChunkBoundaries, CHUNK_SIZE},
    voxel::VoxelTypes,
};

#[derive(Clone)]
pub struct VoxelChunk {
    voxels: Vec<Option<Vec<Option<VoxelTypes>>>>,
    pub count: usize,
    pub lod: i32,
    pub boundary: ChunkBoundaries,
}

impl VoxelChunk {
    pub fn empty(boundary: ChunkBoundaries) -> VoxelChunk {
        VoxelChunk {
            voxels: vec![
                None;
                (CHUNK_SIZE as usize / 8)
                    * (CHUNK_SIZE as usize / 8)
                    * (CHUNK_SIZE as usize / 8)
            ],
            count: 0,
            lod: 1,
            boundary,
        }
    }

    pub fn get_voxels(&self) -> Vec<Voxel> {
        let mut voxels = Vec::with_capacity(self.count);

        for (i, sub_chunk_o) in self.voxels.iter().enumerate() {
            let meta_z = i % 8;
            let meta_y = (i / (CHUNK_SIZE as usize / 8)) % 8;
            let meta_x = (i / ((CHUNK_SIZE as usize / 8) * (CHUNK_SIZE as usize / 8))) % 8;
            if let Some(sub_chunk) = sub_chunk_o {
                for (sub_i, voxel_type_o) in sub_chunk.iter().enumerate() {
                    if let Some(v) = voxel_type_o {
                        let sub_z = sub_i % 8;
                        let sub_y = (sub_i / 8) % 8;
                        let sub_x = (sub_i / 64) % 8;

                        let x = (meta_x * (CHUNK_SIZE as usize / 8) + sub_x) as i32
                            + self.boundary.min[0];
                        let y = (meta_y * (CHUNK_SIZE as usize / 8) + sub_y) as i32
                            + self.boundary.min[1];
                        let z = (meta_z * (CHUNK_SIZE as usize / 8) + sub_z) as i32
                            + self.boundary.min[2];

                        voxels.push(Voxel {
                            position: VoxelPosition { x, y, z },
                            typ: v.clone(),
                        })
                    }
                }
            }
        }
        voxels
    }
    const META_FACTOR: usize = CHUNK_SIZE as usize / 8;

    fn get_vector_position(&self, p: &VoxelPosition) -> (usize, usize) {
        let local_x = (p.x - self.boundary.min[0]) as usize;
        let local_y = (p.y - self.boundary.min[1]) as usize;
        let local_z = (p.z - self.boundary.min[2]) as usize;

        let meta_i = local_z / 8
            + local_y / 8 * VoxelChunk::META_FACTOR
            + local_x / 8 * VoxelChunk::META_FACTOR * VoxelChunk::META_FACTOR;
        let sub_i = local_z % 8 + (local_y % 8) * 8 + (local_x % 8) * 64;

        (meta_i, sub_i)
    }

    fn get_meta_vector_position(&self, p: &VoxelPosition) -> usize {
        let local_x = (p.x - self.boundary.min[0]) as usize;
        let local_y = (p.y - self.boundary.min[1]) as usize;
        let local_z = (p.z - self.boundary.min[2]) as usize;

        local_z / 8
            + local_y / 8 * VoxelChunk::META_FACTOR
            + local_x / 8 * VoxelChunk::META_FACTOR * VoxelChunk::META_FACTOR
    }

    fn get_sub_vector_position(&self, p: &VoxelPosition) -> usize {
        let local_x = (p.x - self.boundary.min[0]) as usize;
        let local_y = (p.y - self.boundary.min[1]) as usize;
        let local_z = (p.z - self.boundary.min[2]) as usize;

        local_z % 8 + (local_y % 8) * 8 + (local_x % 8) * 64
    }

    pub fn set(&mut self, voxel: Voxel) {
        let (meta_i, sub_i) = self.get_vector_position(&voxel.position);
        if let Some(sub_chunk) = &mut self.voxels[meta_i] {
            if sub_chunk[sub_i].is_none() {
                self.count += 1;
            }
            sub_chunk[sub_i] = Some(voxel.typ);
        } else {
            let mut sub_chunk: Vec<Option<VoxelTypes>> = vec![None; 8 * 8 * 8];
            sub_chunk[sub_i] = Some(voxel.typ);
            self.voxels[meta_i] = Some(sub_chunk);
            self.count += 1;
        }
    }

    pub fn remove(&mut self, position: VoxelPosition) -> Option<Voxel> {
        let (meta_i, sub_i) = self.get_vector_position(&position);
        if let Some(sub_chunk) = &mut self.voxels[meta_i] {
            if let Some(typ) = sub_chunk[sub_i] {
                sub_chunk[sub_i] = None;
                self.count -= 1;
                Some(Voxel { position, typ })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get(&self, position: &VoxelPosition) -> Option<VoxelTypes> {
        let meta_i = self.get_meta_vector_position(position);
        if let Some(Some(sub_chunk)) = self.voxels.get(meta_i) {
            let sub_i = self.get_sub_vector_position(position);
            sub_chunk[sub_i]
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::voxel_world::{
        boundaries::ChunkBoundaries, voxel::Voxel, voxel::VoxelPosition, voxel::VoxelTypes,
    };

    use super::VoxelChunk;
    use super::CHUNK_SIZE;

    #[test]
    fn test_chunk() {
        let zero = VoxelPosition { x: 0, y: 0, z: 0 };
        let boundaries = ChunkBoundaries::aligned(zero);
        let mut chunk = VoxelChunk::empty(boundaries);

        assert_eq!(chunk.get_voxels(), Vec::<Voxel>::new());

        let v1 = Voxel {
            position: VoxelPosition { x: 1, y: 2, z: 3 },
            typ: VoxelTypes::BrownRock,
        };
        chunk.set(v1.clone());
        assert_eq!(chunk.get_voxels(), vec![v1]);

        let v1 = Voxel {
            position: VoxelPosition { x: 1, y: 2, z: 3 },
            typ: VoxelTypes::BrownRock,
        };
        chunk.set(v1.clone());
        assert_eq!(chunk.get_voxels(), vec![v1.clone()]);

        let v2 = Voxel {
            position: VoxelPosition { x: 9, y: 2, z: 3 },
            typ: VoxelTypes::BrownRock,
        };
        chunk.set(v2.clone());
        assert_eq!(chunk.get_voxels(), vec![v1.clone(), v2.clone()]);
        assert_eq!(chunk.get(&v2.position), Some(VoxelTypes::BrownRock));

        let v3 = Voxel {
            position: VoxelPosition {
                x: 60,
                y: 60,
                z: 60,
            },
            typ: VoxelTypes::BrownRock,
        };
        chunk.set(v3.clone());
        assert_eq!(chunk.get_voxels(), vec![v1.clone(), v2.clone(), v3.clone()]);
        assert_eq!(chunk.get(&v3.position), Some(VoxelTypes::BrownRock));
    }

    #[test]
    fn test_full_chunk() {
        let zero = VoxelPosition { x: 0, y: 0, z: 0 };
        let boundaries = ChunkBoundaries::aligned(zero);
        let mut chunk = VoxelChunk::empty(boundaries);

        for (x, y, z) in iproduct!(
            (0..CHUNK_SIZE).into_iter(),
            (0..CHUNK_SIZE).into_iter(),
            (0..CHUNK_SIZE).into_iter()
        ) {
            assert_eq!(chunk.get(&VoxelPosition { x, y, z }), None);
            chunk.set(Voxel {
                position: VoxelPosition { x, y, z },
                typ: VoxelTypes::BrownRock,
            });

            assert_eq!(
                chunk.get(&VoxelPosition { x, y, z }),
                Some(VoxelTypes::BrownRock)
            );
        }
        assert_eq!(
            chunk.get_voxels().len(),
            (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize
        );
        let all_voxles = chunk.get_voxels();

        for (x, y, z) in iproduct!(
            (0..CHUNK_SIZE).into_iter(),
            (0..CHUNK_SIZE).into_iter(),
            (0..CHUNK_SIZE).into_iter()
        ) {
            let p = VoxelPosition { x, y, z };

            assert!(all_voxles.iter().find(|v| v.position == p).is_some());
        }
    }
}
