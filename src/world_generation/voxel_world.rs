use ahash::AHashMap;
use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use lerp::Lerp;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::borrow::Cow;

use super::world_structure::*;

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
        asset_server: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for pillar in self.pillars.iter() {
            let m = meshes.add(Mesh::from(pillar));
            let texture = asset_server.load("world_texture_color.png");
            let material = materials.add(StandardMaterial {
                albedo_texture: Some(texture),
                shaded: false,
                ..Default::default()
            });
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
    rock_types: Vec<VoxelTypes>,
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
            rock_types: vec![
                VoxelTypes::DarkRock1,
                VoxelTypes::DarkRock2,
                VoxelTypes::LightRock1,
                VoxelTypes::LightRock2,
                VoxelTypes::CrackedRock,
            ],
        }
    }

    pub fn voxels(&self) -> WorldStructure {
        let mut rng = thread_rng();
        let mut world: WorldStructure = AHashMap::new();
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
                            typ: self.voxel_type(&mut rng, layer),
                        };
                        world.add_voxel(voxel);
                    }
                }
            }
        }
        world
    }

    fn voxel_type(&self, mut rng: &mut ThreadRng, y: i32) -> VoxelTypes {
        if y == self.height - 1 {
            VoxelTypes::Moss
        } else if y == 0 {
            VoxelTypes::Lava
        } else {
            self.rock_types.choose(&mut rng).unwrap().clone()
        }
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
                    let (u_min, u_max, v_min, v_max) = match voxel.typ {
                        VoxelTypes::DarkRock1 => (0.0f32, 0.25f32, 0.5f32, 1.0f32),
                        VoxelTypes::DarkRock2 => (0.25f32, 0.5f32, 0.5f32, 1.0f32),
                        VoxelTypes::Lava => (0.25, 0.5, 0.0, 0.5),
                        VoxelTypes::Moss => (0.5, 0.75, 0.0, 0.5),
                        VoxelTypes::LightRock1 => (0.75, 1.0, 0.0, 0.5),
                        VoxelTypes::LightRock2 => (0.5, 0.75, 0.5, 1.0),
                        VoxelTypes::CrackedRock => (0.0, 0.25, 0.0, 0.5),
                    };

                    let v = &[
                        // top (0., 0., size)
                        (
                            [x - size, y - size, z + size],
                            [0., 0., size],
                            [u_min, v_min],
                        ),
                        (
                            [x + size, y - size, z + size],
                            [0., 0., size],
                            [u_max, v_min],
                        ),
                        (
                            [x + size, y + size, z + size],
                            [0., 0., size],
                            [u_max, v_max],
                        ),
                        (
                            [x - size, y + size, z + size],
                            [0., 0., size],
                            [u_min, v_max],
                        ),
                        // bottom (0., 0., -size)
                        (
                            [x - size, y + size, z - size],
                            [0., 0., -size],
                            [u_max, v_min],
                        ),
                        (
                            [x + size, y + size, z - size],
                            [0., 0., -size],
                            [u_min, v_min],
                        ),
                        (
                            [x + size, y - size, z - size],
                            [0., 0., -size],
                            [u_min, v_max],
                        ),
                        (
                            [x - size, y - size, z - size],
                            [0., 0., -size],
                            [u_max, v_max],
                        ),
                        // right (size, 0., 0.)
                        (
                            [x + size, y - size, z - size],
                            [size, 0., 0.],
                            [u_min, v_min],
                        ),
                        (
                            [x + size, y + size, z - size],
                            [size, 0., 0.],
                            [u_max, v_min],
                        ),
                        (
                            [x + size, y + size, z + size],
                            [size, 0., 0.],
                            [u_max, v_max],
                        ),
                        (
                            [x + size, y - size, z + size],
                            [size, 0., 0.],
                            [u_min, v_max],
                        ),
                        // left (-size, 0., 0.)
                        (
                            [x - size, y - size, z + size],
                            [-size, 0., 0.],
                            [u_max, v_min],
                        ),
                        (
                            [x - size, y + size, z + size],
                            [-size, 0., 0.],
                            [u_min, v_min],
                        ),
                        (
                            [x - size, y + size, z - size],
                            [-size, 0., 0.],
                            [u_min, v_max],
                        ),
                        (
                            [x - size, y - size, z - size],
                            [-size, 0., 0.],
                            [u_max, v_max],
                        ),
                        // front (0., size, 0.)
                        (
                            [x + size, y + size, z - size],
                            [0., size, 0.],
                            [u_max, v_min],
                        ),
                        (
                            [x - size, y + size, z - size],
                            [0., size, 0.],
                            [u_min, v_min],
                        ),
                        (
                            [x - size, y + size, z + size],
                            [0., size, 0.],
                            [u_min, v_max],
                        ),
                        (
                            [x + size, y + size, z + size],
                            [0., size, 0.],
                            [u_max, v_max],
                        ),
                        // back (0., -size, 0.)
                        (
                            [x + size, y - size, z + size],
                            [0., -size, 0.],
                            [u_min, v_min],
                        ),
                        (
                            [x - size, y - size, z + size],
                            [0., -size, 0.],
                            [u_max, v_min],
                        ),
                        (
                            [x - size, y - size, z - size],
                            [0., -size, 0.],
                            [u_max, v_max],
                        ),
                        (
                            [x + size, y - size, z - size],
                            [0., -size, 0.],
                            [u_min, v_max],
                        ),
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
