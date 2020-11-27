use bevy::prelude::*;

use crate::physics::collider::{Collider, ColliderShapes};

use super::voxel::world_2_voxel_space;
use super::world_structure::*;
use crate::vec3_ext::*;

pub fn terrain_collision_system(
    terrain_query: Query<&Terrain>,
    mut movable_colliders_query: Query<(&mut Transform, &Collider)>,
) {
    for (mut transform, collider) in movable_colliders_query.iter_mut() {
        let mut impulse = Vec3::zero();
        let transform_matrix = transform.compute_matrix();
        let transformed_center = transform_matrix.transform_vec3(collider.local_position);

        for terrain in terrain_query.iter() {
            impulse += match collider.collider_shape {
                ColliderShapes::Sphere { radius } => {
                    collision_depth_sphere(terrain, transformed_center, radius)
                }
                ColliderShapes::Cube { half_size } => {
                    collision_depth_cube(terrain, transformed_center, half_size)
                }
                ColliderShapes::Cuboid {
                    half_width_x,
                    half_height_y,
                    half_depth_z,
                } => collision_depth_cubiod(
                    terrain,
                    transformed_center,
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
    let mut max_move = 0.0f32;
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
                        let voxel_min_max = terrain_voxel.position.to_box();
                        let x = voxel_min_max.0.x.max(center.x.min(voxel_min_max.1.x));
                        let y = voxel_min_max.0.y.max(center.y.min(voxel_min_max.1.y));
                        let z = voxel_min_max.0.z.max(center.z.min(voxel_min_max.1.z));
                        let distance = center.distance(Vec3::new(x, y, z));
                        if distance < radius && (radius - distance) > max_move {
                            max_move = radius - distance;
                            let x_distance = (center.x - terrain_voxel.position.x as f32).abs();
                            let y_distance = (center.y - terrain_voxel.position.y as f32).abs();
                            let z_distance = (center.z - terrain_voxel.position.z as f32).abs();
                            if x_distance > y_distance && x_distance > z_distance {
                                overlapping_move = Vec3::new(1.0, 0.0, 0.0) * (radius - distance);
                                if x > center.x {
                                    overlapping_move *= -1.0f32;
                                }
                            }
                            if y_distance > x_distance && y_distance > z_distance {
                                overlapping_move = Vec3::new(0.0, 1.0, 0.0) * (radius - distance);
                                if y > center.y {
                                    overlapping_move *= -1.0f32;
                                }
                            }
                            if z_distance > y_distance && z_distance > x_distance {
                                overlapping_move = Vec3::new(0.0, 0.0, 1.0) * (radius - distance);
                                if z > center.z {
                                    overlapping_move *= -1.0f32;
                                }
                            }
                        }
                    });
            }
        }
    }

    overlapping_move
}

fn collision_depth_cube(terrain: &Terrain, center: Vec3, half_size: f32) -> Vec3 {
    Vec3::zero()
}

fn collision_depth_cubiod(
    terrain: &Terrain,
    center: Vec3,
    half_x: f32,
    half_y: f32,
    half_z: f32,
) -> Vec3 {
    Vec3::zero()
}
