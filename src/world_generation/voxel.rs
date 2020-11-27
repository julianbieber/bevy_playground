use bevy::prelude::*;

pub const HALF_VOXEL_SIZE: f32 = 0.5f32;
const VOXEL_SIZE: f32 = HALF_VOXEL_SIZE * 2.0f32;

#[derive(Clone, Debug)]
pub struct VoxelPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl VoxelPosition {
    pub fn from_vec3(vec3: &Vec3) -> VoxelPosition {
        VoxelPosition {
            x: world_2_voxel_space(vec3.x),
            y: world_2_voxel_space(vec3.y),
            z: world_2_voxel_space(vec3.z),
        }
    }

    pub fn new(x: i32, y: i32, z: i32) -> VoxelPosition {
        VoxelPosition { x, y, z }
    }

    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 * VOXEL_SIZE,
            self.y as f32 * VOXEL_SIZE,
            self.z as f32 * VOXEL_SIZE,
        )
    }

    pub fn to_box(&self) -> (Vec3, Vec3) {
        let center = self.to_vec();
        let min = Vec3::new(
            center.x - HALF_VOXEL_SIZE,
            center.y - HALF_VOXEL_SIZE,
            center.z - HALF_VOXEL_SIZE,
        );
        let max = Vec3::new(
            center.x + HALF_VOXEL_SIZE,
            center.y + HALF_VOXEL_SIZE,
            center.z + HALF_VOXEL_SIZE,
        );
        (min, max)
    }
}

pub fn world_2_voxel_space(s: f32) -> i32 {
    (s / VOXEL_SIZE).ceil() as i32
}

#[derive(Clone, Debug)]
pub struct Voxel {
    pub position: VoxelPosition,
    pub typ: VoxelTypes,
}

impl Voxel {
    pub fn new(x: i32, y: i32, z: i32, typ: VoxelTypes) -> Voxel {
        Voxel {
            position: VoxelPosition { x, y, z },
            typ,
        }
    }
}

#[derive(Clone, Debug)]
pub enum VoxelTypes {
    DarkRock1,
    DarkRock2,
    Lava,
    Moss,
    CrackedRock,
    LightRock1,
    LightRock2,
}
