use ahash::AHashMap;
use std::borrow::Cow;
use std::collections::HashMap;

type YWorldCoordinate = AHashMap<i32, Voxel>;
type ZWorldCoordinate = AHashMap<i32, YWorldCoordinate>;

type WorldStructure = AHashMap<i32, ZWorldCoordinate>;

use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use lerp::Lerp;
use rand::prelude::*;

pub struct VoxelWorld {
    pub pillars: Vec<PillarGenerator>,
}

impl VoxelWorld {
    pub fn generate(width: i32, depth: i32, mut rng: ThreadRng) -> VoxelWorld {
        let pillars: Vec<_> = (0..10)
            .into_iter()
            .map(|_| PillarGenerator::new(&mut rng, width, depth))
            .collect();
        VoxelWorld { pillars }
    }

    pub fn add_to_world(
        &self,
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for pillar in self.pillars.iter() {
            let m = meshes.add(Mesh::from(pillar));
            let material = materials.add(Color::rgb(1.0, 0.0, 0.2).into());
            commands.spawn(PbrComponents {
                mesh: m,
                material,
                ..Default::default()
            });
        }
    }
}

pub struct PillarGenerator {
    position: (i32, i32),
    height: i32,
    upper_radius: i32,
    mid_radius: i32,
    lower_radius: i32,
}

impl PillarGenerator {
    fn new(rng: &mut ThreadRng, width_boundary: i32, depth_boundary: i32) -> PillarGenerator {
        PillarGenerator {
            position: (
                rng.gen_range(width_boundary / -2, width_boundary / 2),
                rng.gen_range(depth_boundary / -2, depth_boundary / 2),
            ),
            height: rng.gen_range(10, 20),
            upper_radius: rng.gen_range(10, 20),
            mid_radius: rng.gen_range(5, 20),
            lower_radius: rng.gen_range(10, 21),
        }
    }

    pub fn voxels(&self) -> WorldStructure {
        let mut blocks: WorldStructure = AHashMap::new();
        for layer in 0..self.height {
            let radius = self.radius_at_level(layer);
            let radius_sq = radius * radius;
            for x in (self.position.0 - radius)..(self.position.0 + radius) {
                for z in (self.position.1 - radius)..(self.position.1 + radius) {
                    let distance_sq = (self.position.0 - x) * (self.position.0 - x)
                        + (self.position.1 - z) * (self.position.1 - z);
                    if distance_sq <= radius_sq {
                        let voxel = Voxel {
                            position: (x, layer, z),
                            typ: VoxelTypes::Rock,
                        };
                        match blocks.get_mut(&x) {
                            Some(z_map) => match z_map.get_mut(&z) {
                                Some(y_map) => {
                                    y_map.insert(layer, voxel);
                                }
                                None => {
                                    let mut y_map = AHashMap::new();
                                    y_map.insert(layer, voxel);
                                    z_map.insert(z, y_map);
                                }
                            },
                            None => {
                                let mut z_map = AHashMap::new();
                                let mut y_map = AHashMap::new();
                                y_map.insert(layer, voxel);
                                z_map.insert(z, y_map);
                                blocks.insert(x, z_map);
                            }
                        };
                    }
                }
            }
        }
        blocks
    }

    fn radius_at_level(&self, level: i32) -> i32 {
        if level < 0 || level > self.height {
            0
        } else if level == 0 {
            self.lower_radius
        } else if level == self.height {
            self.upper_radius
        } else if level < self.height / 2 {
            (self.lower_radius as f32).lerp(
                self.mid_radius as f32,
                level as f32 / (self.height as f32 / 2.0),
            ) as i32
        } else {
            (self.mid_radius as f32).lerp(
                self.upper_radius as f32,
                (level as f32 - self.height as f32 / 2.0f32) / (self.height as f32 / 2.0),
            ) as i32
        }
    }
}

#[derive(Clone, Debug)]
pub struct Voxel {
    pub position: (i32, i32, i32),
    pub typ: VoxelTypes,
}

impl Voxel {
    fn is_next_to(&self, other: &Voxel) -> bool {
        (self.position.0 - other.position.0).abs() <= 1
            && (self.position.1 - other.position.1).abs() <= 1
            && (self.position.2 - other.position.2).abs() <= 1
    }
}

#[derive(Clone, Debug)]
pub enum VoxelTypes {
    Rock,
}

impl From<&PillarGenerator> for Mesh {
    fn from(pillar: &PillarGenerator) -> Self {
        let voxels = pillar.voxels();
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let size = 0.5f32;
        let mut draw_c = 0;

        for (_, x_map) in voxels.iter() {
            for (_, z_map) in x_map.iter() {
                for (_, voxel) in z_map.iter() {
                    draw_c += 1;
                    let x = voxel.position.0 as f32;
                    let y = voxel.position.1 as f32;
                    let z = voxel.position.2 as f32;
                    let v = &[
                        // top (0., 0., size)
                        ([x - size, y - size, z + size], [0., 0., size], [0., 0.]),
                        ([x + size, y - size, z + size], [0., 0., size], [1.0, 0.]),
                        ([x + size, y + size, z + size], [0., 0., size], [1.0, 1.0]),
                        ([x - size, y + size, z + size], [0., 0., size], [0., 1.0]),
                        // bottom (0., 0., -size)
                        ([x - size, y + size, z - size], [0., 0., -size], [1.0, 0.]),
                        ([x + size, y + size, z - size], [0., 0., -size], [0., 0.]),
                        ([x + size, y - size, z - size], [0., 0., -size], [0., 1.0]),
                        ([x - size, y - size, z - size], [0., 0., -size], [1.0, 1.0]),
                        // right (size, 0., 0.)
                        ([x + size, y - size, z - size], [size, 0., 0.], [0., 0.]),
                        ([x + size, y + size, z - size], [size, 0., 0.], [1.0, 0.]),
                        ([x + size, y + size, z + size], [size, 0., 0.], [1.0, 1.0]),
                        ([x + size, y - size, z + size], [size, 0., 0.], [0., 1.0]),
                        // left (-size, 0., 0.)
                        ([x - size, y - size, z + size], [-size, 0., 0.], [size, 0.]),
                        ([x - size, y + size, z + size], [-size, 0., 0.], [0., 0.]),
                        ([x - size, y + size, z - size], [-size, 0., 0.], [0., 1.0]),
                        ([x - size, y - size, z - size], [-size, 0., 0.], [1.0, 1.0]),
                        // front (0., size, 0.)
                        ([x + size, y + size, z - size], [0., size, 0.], [1.0, 0.]),
                        ([x - size, y + size, z - size], [0., size, 0.], [0., 0.]),
                        ([x - size, y + size, z + size], [0., size, 0.], [0., 1.0]),
                        ([x + size, y + size, z + size], [0., size, 0.], [1.0, 1.0]),
                        // back (0., -size, 0.)
                        ([x + size, y - size, z + size], [0., -size, 0.], [0., 0.]),
                        ([x - size, y - size, z + size], [0., -size, 0.], [1.0, 0.]),
                        ([x - size, y - size, z - size], [0., -size, 0.], [1.0, 1.0]),
                        ([x + size, y - size, z - size], [0., -size, 0.], [0., 1.0]),
                    ];

                    for (position, normal, uv) in v.iter() {
                        vertices.push(*position);
                        normals.push(*normal);
                        uvs.push(*uv);
                    }

                    let i = vertices.len() - v.len();
                    let offset = i as u32;
                    let local_indices = [
                        offset + 0,
                        offset + 1,
                        offset + 2,
                        offset + 2,
                        offset + 3,
                        offset + 0, // top
                        offset + 4,
                        offset + 5,
                        offset + 6,
                        offset + 6,
                        offset + 7,
                        offset + 4, // bottom
                        offset + 8,
                        offset + 9,
                        offset + 10,
                        offset + 10,
                        offset + 11,
                        offset + 8, // right
                        offset + 12,
                        offset + 13,
                        offset + 14,
                        offset + 14,
                        offset + 15,
                        offset + 12, // left
                        offset + 16,
                        offset + 17,
                        offset + 18,
                        offset + 18,
                        offset + 19,
                        offset + 16, // front
                        offset + 20,
                        offset + 21,
                        offset + 22,
                        offset + 22,
                        offset + 23,
                        offset + 20, // back
                    ];
                    indices.extend(local_indices.iter());
                }
            }
        }
        dbg!(draw_c);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_POSITION), vertices.into());
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_NORMAL), normals.into());
        mesh.set_attribute(Cow::Borrowed(Mesh::ATTRIBUTE_UV_0), uvs.into());
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
