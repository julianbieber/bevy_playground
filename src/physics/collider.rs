use bevy::ecs::Query;
use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::AddAssign;

pub enum ColliderShapes {
    Sphere {
        radius: f32,
    },
    Cuboid {
        half_width_x: f32,
        half_height_y: f32,
        half_depth_z: f32,
    },
}

impl ColliderShapes {
    pub fn cube(side_length: f32) -> ColliderShapes {
        let half_side = side_length / 2.0f32;

        ColliderShapes::Cuboid {
            half_width_x: half_side,
            half_height_y: half_side,
            half_depth_z: half_side,
        }
    }
}

pub fn cuboid_vertices(
    center: &Vec3,
    transform_matrix: &Mat4,
    half_x: f32,
    half_y: f32,
    half_z: f32,
) -> Vec<Vec3> {
    let mut vertices = Vec::new();
    vertices.reserve(8);
    for x in [center.x - half_x, center.x + half_x].iter() {
        for y in [center.y - half_y, center.y + half_y].iter() {
            for z in [center.z - half_z, center.z + half_z].iter() {
                vertices.push(transform_matrix.transform_point3(Vec3::new(
                    x.clone(),
                    y.clone(),
                    z.clone(),
                )));
            }
        }
    }
    vertices
}

pub struct Collider {
    pub collider_shape: ColliderShapes,
    pub local_position: Vec3,
}

impl Collider {
    fn detect_collision(
        &self,
        other: &Collider,
        transform: &Mat4,
        other_transform: &Mat4,
    ) -> Option<Vec3> {
        match self.collider_shape {
            ColliderShapes::Sphere { radius } => {
                Collider::collision_sphere(self, radius, other, transform, other_transform)
            }
            ColliderShapes::Cuboid {
                half_width_x: _,
                half_height_y: _,
                half_depth_z: _,
            } => None,
        }
    }
    fn collision_sphere(
        &self,
        self_radius: f32,
        other: &Collider,
        transform: &Mat4,
        other_transform: &Mat4,
    ) -> Option<Vec3> {
        let self_to_other = transform.mul_vec4(Vec4::new(
            self.local_position.x,
            self.local_position.y,
            self.local_position.z,
            1.0,
        )) - other_transform.mul_vec4(Vec4::new(
            other.local_position.x,
            other.local_position.y,
            other.local_position.z,
            1.0,
        ));
        match other.collider_shape {
            ColliderShapes::Sphere { radius } => {
                if self_to_other.length() > radius + self_radius {
                    None
                } else {
                    let impulse_strength = 0.5 * ((radius + self_radius) - self_to_other.length())
                        / self_to_other.length();
                    Option::Some(
                        -(Vec3::new(self_to_other.x, self_to_other.y, self_to_other.z)
                            * impulse_strength),
                    )
                }
            }
            ColliderShapes::Cuboid {
                half_width_x,
                half_height_y,
                half_depth_z,
            } => {
                let local_sphere_center =
                    other_transform
                        .inverse()
                        .mul_vec4(transform.mul_vec4(Vec4::new(
                            self.local_position.x,
                            self.local_position.y,
                            self.local_position.z,
                            1.0,
                        )));
                let closest_x = (other.local_position.x - half_width_x).max(
                    local_sphere_center
                        .x
                        .min(other.local_position.x + half_width_x),
                );
                let closest_y = (other.local_position.y - half_height_y).max(
                    local_sphere_center
                        .y
                        .min(other.local_position.y + half_height_y),
                );
                let closest_z = (other.local_position.z - half_depth_z).max(
                    local_sphere_center
                        .z
                        .min(other.local_position.z + half_depth_z),
                );
                let offset: Vec4 =
                    local_sphere_center - Vec4::new(closest_x, closest_y, closest_z, 1.0);
                if !(offset.length() < self_radius) || offset.length() == 0.0 {
                    None
                } else {
                    let impulse_strength =
                        0.5 * ((self_radius - offset.length()) / offset.length());
                    Option::Some(-Vec3::new(offset.x, offset.y, offset.z) * impulse_strength)
                }
            }
        }
    }
}

pub fn collision_update(mut query: Query<(&Collider, &mut Transform)>) {
    let mut colliders = Vec::new();
    for (collider, transform) in query.iter_mut() {
        colliders.push((collider.clone(), transform.compute_matrix()));
    }
    let mut impulses = HashMap::new();
    for (collider, collider_transform) in colliders.iter() {
        let key = format!(
            "{:?}",
            collider_transform.mul_vec4(Vec4::new(
                collider.local_position.x,
                collider.local_position.y,
                collider.local_position.z,
                1.0
            ))
        );

        for (other_collider, other_transform) in colliders.iter() {
            if !other_transform.eq(collider_transform) {
                let impulse = collider
                    .detect_collision(other_collider, collider_transform, other_transform)
                    .unwrap_or(Vec3::zero());
                impulses
                    .entry(key.clone())
                    .or_insert(Vec3::zero())
                    .function(impulse);
            }
        }
    }
    &impulses;
    for (collider, mut collider_transform) in query.iter_mut() {
        let key = format!(
            "{:?}",
            collider_transform.compute_matrix().mul_vec4(Vec4::new(
                collider.local_position.x,
                collider.local_position.y,
                collider.local_position.z,
                1.0f32
            ))
        );
        collider_transform.translation =
            collider_transform.translation - *impulses.get(&key).unwrap();
        collider_transform.translation;
    }
}

trait Vec3Ext {
    fn function(&mut self, vec: Vec3);
}

impl Vec3Ext for Vec3 {
    fn function(&mut self, vec: Vec3) {
        self.add_assign(vec);
    }
}
