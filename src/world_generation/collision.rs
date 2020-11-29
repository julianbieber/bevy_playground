use bevy::prelude::*;

use crate::physics::collider::{cuboid_vertices, Collider, ColliderShapes};

use super::voxel::{world_2_voxel_space, VoxelBox};
use super::world_structure::*;
use std::cmp::Ordering::Equal;

pub fn terrain_collision_system(
    terrain_query: Query<&Terrain>,
    mut movable_colliders_query: Query<(&mut Transform, &Collider)>,
) {
    for (mut transform, collider) in movable_colliders_query.iter_mut() {
        let mut impulse = Vec3::zero();
        let transform_matrix = transform.compute_matrix();
        let transformed_center = transform_matrix.transform_point3(collider.local_position);

        for terrain in terrain_query.iter() {
            impulse += match collider.collider_shape {
                ColliderShapes::Sphere { radius } => {
                    collision_depth_sphere(terrain, transformed_center, radius)
                }
                ColliderShapes::Cuboid {
                    half_width_x,
                    half_height_y,
                    half_depth_z,
                } => collision_depth_cubiod(
                    terrain,
                    collider.local_position,
                    transform_matrix,
                    half_width_x,
                    half_height_y,
                    half_depth_z,
                ),
            };
        }
        transform.translation += impulse;
    }
}

fn collision_depth_sphere(terrain: &Terrain, center: Vec3, radius: f32) -> Vec3 {
    let mut overlapping_move = Vec3::zero();
    for potential_x in
        world_2_voxel_space(center.x - radius) - 1..world_2_voxel_space(center.x + radius) + 1
    {
        for potential_y in
            world_2_voxel_space(center.y - radius) - 1..world_2_voxel_space(center.y + radius) + 1
        {
            for potential_z in world_2_voxel_space(center.z - radius) - 1
                ..world_2_voxel_space(center.z + radius) + 1
            {
                terrain
                    .structure
                    .get_at(&potential_x, &potential_y, &potential_z)
                    .map(|terrain_voxel| {
                        let closest_point = terrain_voxel.position.to_box().closest_point(&center);
                        let voxel_world_position = terrain_voxel.position.to_vec();
                        let distance = center.distance(closest_point);
                        if distance < radius {
                            let x_distance = (center.x - voxel_world_position.x).abs();
                            let y_distance = (center.y - voxel_world_position.y).abs();
                            let z_distance = (center.z - voxel_world_position.z).abs();
                            let move_length = radius - distance;
                            if x_distance >= y_distance
                                && x_distance >= z_distance
                                && overlapping_move.x.abs() < move_length
                            {
                                if closest_point.x < center.x {
                                    overlapping_move.x = move_length;
                                } else {
                                    overlapping_move.x = -move_length;
                                }
                            }

                            if y_distance >= x_distance
                                && y_distance >= z_distance
                                && overlapping_move.y.abs() < move_length
                            {
                                if closest_point.y < center.y {
                                    overlapping_move.y = move_length;
                                } else {
                                    overlapping_move.y = -move_length;
                                }
                            }

                            if z_distance >= x_distance
                                && z_distance >= y_distance
                                && overlapping_move.z.abs() < move_length
                            {
                                if closest_point.z < center.z {
                                    overlapping_move.z = move_length;
                                } else {
                                    overlapping_move.z = -move_length;
                                }
                            }
                        }
                    });
            }
        }
    }

    overlapping_move
}

fn collision_depth_cubiod(
    terrain: &Terrain,
    center: Vec3,
    transform: Mat4,
    half_x: f32,
    half_y: f32,
    half_z: f32,
) -> Vec3 {
    let object_vertices = cuboid_vertices(&center, &transform, half_x, half_y, half_z);
    let transformed_center = transform.transform_point3(center);
    let mut movement = Vec3::zero();
    let mut max_distacne = 0.0f32;
    for potential_x in world_2_voxel_space(transformed_center.x - half_x) - 1
        ..world_2_voxel_space(transformed_center.x + half_x) + 1
    {
        for potential_y in world_2_voxel_space(transformed_center.y - half_y) - 1
            ..world_2_voxel_space(transformed_center.y + half_y) + 1
        {
            for potential_z in world_2_voxel_space(transformed_center.z - half_z) - 1
                ..world_2_voxel_space(transformed_center.z + half_z) + 1
            {
                terrain
                    .structure
                    .get_at(&potential_x, &potential_y, &potential_z)
                    .map(|terrain_voxel| {
                        let voxel_world_position = terrain_voxel.position.to_vec();
                        let voxel_vertices = terrain_voxel.position.vertices();

                        let object_distances = object_vertices
                            .iter()
                            .map(|v| {
                                point_on_line(transformed_center, voxel_world_position, v.clone())
                            })
                            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
                            .unwrap();
                        let voxel_distances = voxel_vertices
                            .iter()
                            .map(|v| {
                                point_on_line(transformed_center, voxel_world_position, v.clone())
                            })
                            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
                            .unwrap();

                        if object_distances > voxel_distances
                            && object_distances > voxel_distances - object_distances
                        {
                            max_distacne = voxel_distances - object_distances;
                            movement += (voxel_world_position - transformed_center).normalize()
                                * (voxel_distances - object_distances);
                        }
                    });
            }
        }
    }

    movement
}

fn point_on_line(a: Vec3, b: Vec3, p: Vec3) -> f32 {
    let ap = p - a;
    let ab = b - a;

    let projected_p = a + ap.dot(ab) / ab.dot(ab) * ab;
    a.distance(projected_p)
}
