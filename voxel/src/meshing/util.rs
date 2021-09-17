use bevy::math::Vec3;

use crate::voxel::{VoxelDirection, VoxelTypes, HALF_VOXEL_SIZE};

pub fn to_vertices(direction: VoxelDirection, center: Vec3) -> [[f32; 3]; 4] {
    match direction {
        VoxelDirection::UP => [
            [
                center.x - HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
            [
                center.x - HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x + HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x + HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
        ],
        VoxelDirection::DOWN => [
            [
                center.x + HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
            [
                center.x + HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x - HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x - HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
        ],
        VoxelDirection::LEFT => [
            [
                center.x - HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
            [
                center.x - HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x - HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x - HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
        ],
        VoxelDirection::RIGHT => [
            [
                center.x + HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
            [
                center.x + HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x + HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x + HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
        ],
        VoxelDirection::FRONT => [
            [
                center.x - HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
            [
                center.x - HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
            [
                center.x + HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
            [
                center.x + HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z - HALF_VOXEL_SIZE,
            ],
        ],
        VoxelDirection::BACK => [
            [
                center.x + HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x + HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x - HALF_VOXEL_SIZE,
                center.y + HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
            [
                center.x - HALF_VOXEL_SIZE,
                center.y - HALF_VOXEL_SIZE,
                center.z + HALF_VOXEL_SIZE,
            ],
        ],
    }
}

pub fn to_normals(direction: VoxelDirection) -> [[f32; 3]; 4] {
    match direction {
        VoxelDirection::UP => [[0.0, 1.0, 0.0]; 4],
        VoxelDirection::DOWN => [[0.0, -1.0, 0.0]; 4],
        VoxelDirection::LEFT => [[-1.0, 0.0, 0.0]; 4],
        VoxelDirection::RIGHT => [[1.0, 0.0, 0.0]; 4],
        VoxelDirection::FRONT => [[0.0, 0.0, -1.0]; 4],
        VoxelDirection::BACK => [[0.0, 0.0, 1.0]; 4],
    }
}

pub fn to_tangents(direction: VoxelDirection) -> [[f32; 4]; 4] {
    match direction {
        VoxelDirection::UP => [[1.0, 0.0, 0.0, -1.0]; 4],
        VoxelDirection::DOWN => [[1.0, 0.0, 0.0, 1.0]; 4],
        VoxelDirection::LEFT => [[0.0, 0.0, 1.0, 1.0]; 4],
        VoxelDirection::RIGHT => [[0.0, 0.0, 1.0, -1.0]; 4],
        VoxelDirection::FRONT => [[1.0, 0.0, 0.0, -1.0]; 4],
        VoxelDirection::BACK => [[1.0, 0.0, 0.0, 1.0]; 4],
    }
}

pub fn uvs_from_typ(typ: &VoxelTypes) -> [[f32; 2]; 4] {
    let (u_min, u_max, v_min, v_max) = match typ {
        VoxelTypes::DarkRock1 => (0.0, 0.125, 0.0, 1.0),
        VoxelTypes::Moss => (0.125, 0.25, 0.0, 1.0),
        VoxelTypes::GreyRock1 => (0.25, 0.375, 0.0, 1.0),
        VoxelTypes::GreyRock2 => (0.375, 0.5, 0.0, 1.0),
        VoxelTypes::BrownRock => (0.5, 0.625, 0.0, 1.0),
        VoxelTypes::DarkRock2 => (0.625, 0.75, 0.0, 1.0),
        VoxelTypes::GroundRock1 => (0.75, 0.875, 0.0, 1.0),
        VoxelTypes::Snow => (0.875, 1.0, 0.0, 1.0),
    };
    [
        [u_min, v_min],
        [u_min, v_max],
        [u_max, v_min],
        [u_max, v_max],
    ]
}
