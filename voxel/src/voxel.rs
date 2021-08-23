use std::ops::{Add, Div, Mul, Sub};

use bevy::prelude::*;
use strum_macros::EnumIter;

pub const HALF_VOXEL_SIZE: f32 = 0.5f32;
pub const VOXEL_SIZE: f32 = HALF_VOXEL_SIZE * 2.0f32;

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
pub enum VoxelDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    FRONT,
    BACK,
}

#[derive(Debug)]
pub struct VoxelFace {
    pub direction: VoxelDirection,
    pub center: Vec3,
    pub size: f32,
    pub typ: VoxelTypes,
}

impl VoxelFace {
    pub fn from_voxels(
        center: Vec3,
        typ: VoxelTypes,
        direction: VoxelDirection,
        lod: i32,
    ) -> VoxelFace {
        VoxelFace {
            center,
            size: lod as f32 * HALF_VOXEL_SIZE,
            typ,
            direction,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct VoxelPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Add for VoxelPosition {
    type Output = VoxelPosition;

    fn add(self, rhs: Self) -> Self::Output {
        VoxelPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for VoxelPosition {
    type Output = VoxelPosition;

    fn sub(self, rhs: Self) -> Self::Output {
        VoxelPosition {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<i32> for VoxelPosition {
    type Output = VoxelPosition;

    fn div(self, rhs: i32) -> Self::Output {
        VoxelPosition {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<i32> for VoxelPosition {
    type Output = VoxelPosition;

    fn mul(self, rhs: i32) -> Self::Output {
        VoxelPosition {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[derive(Debug)]
pub struct VoxelSurrounding {
    pub top: VoxelPosition,
    pub bottom: VoxelPosition,
    pub left: VoxelPosition,
    pub right: VoxelPosition,
    pub front: VoxelPosition,
    pub back: VoxelPosition,
}

impl VoxelSurrounding {
    pub fn vec(&self) -> Vec<VoxelPosition> {
        vec![
            self.top.clone(),
            self.bottom.clone(),
            self.left.clone(),
            self.right.clone(),
            self.front.clone(),
            self.back.clone(),
        ]
    }
}

impl VoxelPosition {
    pub fn from_vec3(vec3: &Vec3) -> VoxelPosition {
        VoxelPosition {
            x: world_2_voxel_space(vec3.x),
            y: world_2_voxel_space(vec3.y),
            z: world_2_voxel_space(vec3.z),
        }
    }

    pub fn up_to(x: i32, y_top: i32, z: i32) -> Vec<VoxelPosition> {
        (-60..y_top)
            .into_iter()
            .map(|y| VoxelPosition::new(x, y, z))
            .collect()
    }

    pub fn new(x: i32, y: i32, z: i32) -> VoxelPosition {
        VoxelPosition { x, y, z }
    }

    pub fn diagonal(a: i32) -> VoxelPosition {
        VoxelPosition { x: a, y: a, z: a }
    }

    pub fn voxel_distance(d: f32) -> i32 {
        (d / VOXEL_SIZE) as i32
    }

    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 * VOXEL_SIZE,
            self.y as f32 * VOXEL_SIZE,
            self.z as f32 * VOXEL_SIZE,
        )
    }

    pub fn transform(&self) -> Mat4 {
        Transform::from_translation(self.to_vec()).compute_matrix()
    }

    pub fn in_direction(&self, direction: VoxelDirection) -> VoxelPosition {
        match direction {
            VoxelDirection::UP => VoxelPosition {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            VoxelDirection::DOWN => VoxelPosition {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            VoxelDirection::LEFT => VoxelPosition {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            VoxelDirection::RIGHT => VoxelPosition {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            VoxelDirection::FRONT => VoxelPosition {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            VoxelDirection::BACK => VoxelPosition {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        }
    }

    pub fn surrounding(&self) -> VoxelSurrounding {
        VoxelSurrounding {
            top: self.in_direction(VoxelDirection::UP),
            bottom: self.in_direction(VoxelDirection::DOWN),
            left: self.in_direction(VoxelDirection::LEFT),
            right: self.in_direction(VoxelDirection::RIGHT),
            front: self.in_direction(VoxelDirection::FRONT),
            back: self.in_direction(VoxelDirection::BACK),
        }
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

    pub fn sphere(center: &Vec3, radius: f32) -> Vec<VoxelPosition> {
        let center_voxel = VoxelPosition::from_vec3(center);
        let voxel_radius = (radius / VOXEL_SIZE) as i32;
        let mut voxels = Vec::with_capacity((voxel_radius + voxel_radius * voxel_radius) as usize);
        for x in center_voxel.x - voxel_radius..center_voxel.x + voxel_radius {
            for y in center_voxel.y - voxel_radius..center_voxel.y + voxel_radius {
                for z in center_voxel.z - voxel_radius..center_voxel.z + voxel_radius {
                    voxels.push(VoxelPosition { x, y, z });
                }
            }
        }
        voxels
    }
}

pub trait VoxelBox {
    fn closest_point(&self, other: &Vec3) -> Vec3;
}

impl VoxelBox for (Vec3, Vec3) {
    fn closest_point(&self, other: &Vec3) -> Vec3 {
        let x = self.0.x.max(other.x.min(self.1.x));
        let y = self.0.y.max(other.y.min(self.1.y));
        let z = self.0.z.max(other.z.min(self.1.z));
        Vec3::new(x, y, z)
    }
}

pub fn world_2_voxel_space(s: f32) -> i32 {
    (s / VOXEL_SIZE).ceil() as i32
}

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub enum VoxelTypes {
    Moss,
    DarkRock1,
    GreyRock1,
    GreyRock2,
    BrownRock,
    DarkRock2,
    GroundRock1,
    Snow,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Voxel {
    LandVoxel { typ: VoxelTypes },
    WaterVoxel { fill: f32, used_indices: Vec<usize> },
    Nothing,
}
