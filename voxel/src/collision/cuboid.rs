use bevy::prelude::*;
use cgmath::Matrix4;
use collision::{algorithm::minkowski::GJK3, primitive::Cuboid, CollisionStrategy};

use crate::{access::VoxelAccess, voxel::VoxelPosition};

use crate::voxel::{world_2_voxel_space, HALF_VOXEL_SIZE};

pub fn collision_depth_cubiod(
    voxel_access: &VoxelAccess,
    center: Vec3,
    transform: Mat4,
    half_x: f32,
    half_y: f32,
    half_z: f32,
) -> Vec3 {
    let transformed_center = transform.transform_point3(center);
    let mut movement = Vec3::ZERO;
    let mut max_distance = 0.0f32;

    for potential_x in world_2_voxel_space(transformed_center.x - half_x) - 1
        ..world_2_voxel_space(transformed_center.x + half_x) + 1
    {
        for potential_y in world_2_voxel_space(transformed_center.y - half_y) - 1
            ..world_2_voxel_space(transformed_center.y + half_y) + 1
        {
            for potential_z in world_2_voxel_space(transformed_center.z - half_z) - 1
                ..world_2_voxel_space(transformed_center.z + half_z) + 1
            {
                let position = VoxelPosition {
                    x: potential_x,
                    y: potential_y,
                    z: potential_z,
                };
                if let Some(chunk) = voxel_access.get_chunk_containing(position) {
                    chunk.get(&position).map(|_| {
                        if let Some((distance, axis)) = collision(
                            Vec3::new(half_x, half_y, half_z),
                            &transform,
                            Vec3::new(HALF_VOXEL_SIZE, HALF_VOXEL_SIZE, HALF_VOXEL_SIZE),
                            &position.transform(),
                        ) {
                            if distance > max_distance {
                                movement = axis * distance;
                                max_distance = distance;
                            }
                        }
                    });
                }
            }
        }
    }
    movement * -1.0f32
}

fn collision(
    object_size: Vec3,
    object_transform: &Mat4,
    terrain_size: Vec3,
    terrain_transform: &Mat4,
) -> Option<(f32, Vec3)> {
    let gjk = GJK3::new();
    let object_cuboid = Cuboid::new(
        object_size.x * 2.0,
        object_size.y * 2.0,
        object_size.z * 2.0,
    );
    let terrain_cuboid = Cuboid::new(
        terrain_size.x * 2.0,
        terrain_size.y * 2.0,
        terrain_size.z * 2.0,
    );
    let left_transform = to_cgmath(object_transform);
    let right_transform = to_cgmath(terrain_transform);
    gjk.intersection(
        &CollisionStrategy::FullResolution,
        &object_cuboid,
        &left_transform,
        &terrain_cuboid,
        &right_transform,
    )
    .map(|contact| {
        let normal = contact.normal;
        (
            contact.penetration_depth,
            Vec3::new(normal.x, normal.y, normal.z),
        )
    })
}

fn to_cgmath(m: &Mat4) -> Matrix4<f32> {
    Matrix4::new(
        m.x_axis.x, m.x_axis.y, m.x_axis.z, m.x_axis.w, m.y_axis.x, m.y_axis.y, m.y_axis.z,
        m.y_axis.w, m.z_axis.x, m.z_axis.y, m.z_axis.z, m.z_axis.w, m.w_axis.x, m.w_axis.y,
        m.w_axis.z, m.w_axis.w,
    )
}
