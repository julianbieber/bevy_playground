use crate::voxel::{Voxel, VoxelPosition};

use super::{
    boundaries::{ChunkBoundaries, CHUNK_SIZE},
    voxel::VoxelTypes,
};

#[derive(Debug, Clone)]
pub struct VoxelChunk {
    voxels: Vec<Option<VoxelTypes>>,
    pub count: usize,
    pub lod: i32,
    pub boundary: ChunkBoundaries,
}

impl VoxelChunk {
    pub fn empty(boundary: ChunkBoundaries) -> VoxelChunk {
        VoxelChunk {
            voxels: vec![],
            count: 0,
            lod: 1,
            boundary,
        }
    }

    pub fn filter<A>(&self, f: A) -> Vec<Voxel>
    where
        A: Fn(&Voxel) -> bool,
    {
        let mut voxels = Vec::with_capacity(self.count);

        for (i, voxel_type_o) in self.voxels.iter().enumerate() {
            if let Some(v) = voxel_type_o {
                let voxel = Voxel {
                    position: self.index_to_coord(i),
                    typ: v.clone(),
                };
                if f(&voxel) {
                    voxels.push(voxel);
                }
            }
        }
        voxels
    }

    pub fn get_voxels(&self) -> Vec<Voxel> {
        let mut voxels = Vec::with_capacity(self.count);

        for (i, voxel_type_o) in self.voxels.iter().enumerate() {
            if let Some(v) = voxel_type_o {
                let voxel = Voxel {
                    position: self.index_to_coord(i),
                    typ: v.clone(),
                };
                voxels.push(voxel);
            }
        }
        voxels
    }

    fn index_to_coord(&self, i: usize) -> VoxelPosition {
        let z = i as i32 / (CHUNK_SIZE * CHUNK_SIZE);
        let y = (i as i32 / CHUNK_SIZE) % CHUNK_SIZE;
        let x = i as i32 % CHUNK_SIZE;
        VoxelPosition {
            x: x + self.boundary.min[0],
            y: y + self.boundary.min[2],
            z: z + self.boundary.min[2],
        }
    }

    fn get_vector_position(&self, p: &VoxelPosition) -> usize {
        let x = p.x - self.boundary.min[0];
        let y = p.y - self.boundary.min[1];
        let z = p.z - self.boundary.min[2];
        (z * CHUNK_SIZE * CHUNK_SIZE + y * CHUNK_SIZE + x) as usize
    }

    pub fn set(&mut self, voxel: Voxel) {
        if !self.boundary.contains(&voxel.position) {
            panic!(
                "Tried to set voxel for {:?}. ChunkBoundaries: {:?}",
                voxel.position, self.boundary
            );
        }

        let i = self.get_vector_position(&voxel.position);
        if self.count == 0 {
            self.voxels = vec![None; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize];
            self.count = 1;
            self.voxels[i] = Some(voxel.typ);
        } else {
            if self.voxels[i].is_none() {
                self.count += 1;
            }
            self.voxels[i] = Some(voxel.typ);
        }
    }

    pub fn remove(&mut self, position: VoxelPosition) -> Option<Voxel> {
        if self.count == 0 || !self.boundary.contains(&position) {
            None
        } else {
            let i = self.get_vector_position(&position);
            if let Some(typ) = self.voxels[i] {
                self.voxels[i] = None;
                self.count -= 1;
                Some(Voxel { position, typ })
            } else {
                None
            }
        }
    }

    pub fn get(&self, position: &VoxelPosition) -> Option<VoxelTypes> {
        if !self.boundary.contains(position) {
            panic!(
                "Tried to get voxel for {:?}. ChunkBoundaries: {:?}",
                position, self.boundary
            );
        }
        if self.count == 0 {
            None
        } else {
            let i = self.get_vector_position(&position);
            if let Some(typ) = self.voxels[i] {
                Some(typ)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::iproduct;

    use crate::{
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

    #[test]
    fn boundary_over_position_and_contains_consistency() {
        let position = VoxelPosition {
            x: -45,
            y: -120,
            z: -93,
        };

        let alingned = ChunkBoundaries::aligned(position);

        let matching_boundary = ChunkBoundaries {
            min: [-64, -128, -128],
            max: [0, -64, -64],
        };

        assert_eq!(alingned, matching_boundary);
    }
}
