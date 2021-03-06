use bevy::ecs::Query;
use bevy::prelude::*;
use cgmath::num_traits::Float;
use itertools::Itertools;
use std::cmp;
use std::cmp::min;
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

pub fn cuboid_edges_untransformed() -> Vec<Vec3> {
    let top_left_front = Vec3::new(-1.0, 1.0, 1.0);
    let top_left_back = Vec3::new(-1.0, 1.0, -1.0);
    let lower_left_front = Vec3::new(-1.0, -1.0, 1.0);
    let top_right_front = Vec3::new(1.0, 1.0, 1.0);
    vec![
        top_left_back - top_left_front,
        top_right_front - top_left_front,
        lower_left_front - top_left_front,
    ]
}

pub fn cuboid_edges(transform_matrix: &Mat4) -> Vec<Vec3> {
    cuboid_edges_untransformed()
        .iter()
        .map(|v| transform_matrix.transform_vector3(*v))
        .collect()
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
                half_width_x,
                half_height_y,
                half_depth_z,
            } => Collider::collision_cuboid(
                self,
                half_width_x,
                half_height_y,
                half_depth_z,
                other,
                transform,
                other_transform,
            ),
        }
    }

    fn collision_sphere(
        &self,
        self_radius: f32,
        other: &Collider,
        transform: &Mat4,
        other_transform: &Mat4,
    ) -> Option<Vec3> {
        let self_to_other =
            Collider::midpoint_to_other_midpoint(&self, transform, other, other_transform);
        match other.collider_shape {
            ColliderShapes::Sphere { radius } => {
                Collider::collision_sphere_sphere(self_to_other, radius, self_radius)
            }
            ColliderShapes::Cuboid {
                half_width_x,
                half_height_y,
                half_depth_z,
            } => Collider::collision_sphere_cuboid(
                &self,
                self_radius,
                other,
                transform,
                other_transform,
                half_width_x,
                half_height_y,
                half_depth_z,
            ),
        }
    }

    fn midpoint_to_other_midpoint(
        &self,
        transform: &Mat4,
        other: &Collider,
        other_transform: &Mat4,
    ) -> Vec4 {
        transform.mul_vec4(Vec4::new(
            self.local_position.x,
            self.local_position.y,
            self.local_position.z,
            1.0,
        )) - other_transform.mul_vec4(Vec4::new(
            other.local_position.x,
            other.local_position.y,
            other.local_position.z,
            1.0,
        ))
    }

    fn collision_sphere_sphere(self_to_other: Vec4, radius: f32, self_radius: f32) -> Option<Vec3> {
        if self_to_other.length() > radius + self_radius {
            None
        } else {
            let impulse_strength =
                ((radius + self_radius) - self_to_other.length()) / self_to_other.length();
            Option::Some(
                -(Vec3::new(self_to_other.x, self_to_other.y, self_to_other.z) * impulse_strength),
            )
        }
    }

    fn collision_sphere_cuboid(
        &self,
        self_radius: f32,
        other: &Collider,
        transform: &Mat4,
        other_transform: &Mat4,
        half_width_x: f32,
        half_height_y: f32,
        half_depth_z: f32,
    ) -> Option<Vec3> {
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
        let offset: Vec4 = local_sphere_center - Vec4::new(closest_x, closest_y, closest_z, 1.0);
        if !(offset.length() < self_radius) || offset.length() == 0.0 {
            None
        } else {
            let impulse_strength = (self_radius - offset.length()) / offset.length();
            Option::Some(-Vec3::new(offset.x, offset.y, offset.z) * impulse_strength)
        }
    }

    fn collision_cuboid(
        &self,
        self_half_width_x: f32,
        self_half_height_y: f32,
        self_half_depth_z: f32,
        other: &Collider,
        transform: &Mat4,
        other_transform: &Mat4,
    ) -> Option<Vec3> {
        match other.collider_shape {
            ColliderShapes::Sphere { radius } => Collider::collision_cuboid_sphere(
                &self,
                self_half_width_x,
                self_half_height_y,
                self_half_depth_z,
                other,
                transform,
                other_transform,
                radius,
            ),
            ColliderShapes::Cuboid {
                half_width_x,
                half_height_y,
                half_depth_z,
            } => {
                Collider::gjk(
                    self_half_width_x,
                    self_half_height_y,
                    self_half_depth_z,
                    self.local_position,
                    half_width_x,
                    half_height_y,
                    half_depth_z,
                    other.local_position,
                    transform,
                    other_transform,
                );
                None
            }
        }
    }

    fn collision_cuboid_sphere(
        &self,
        self_half_width_x: f32,
        self_half_height_y: f32,
        self_half_depth_z: f32,
        other: &Collider,
        transform: &Mat4,
        other_transform: &Mat4,
        radius: f32,
    ) -> Option<Vec3> {
        let local_sphere_center =
            transform
                .inverse()
                .mul_vec4(other_transform.mul_vec4(Vec4::new(
                    other.local_position.x,
                    other.local_position.y,
                    other.local_position.z,
                    1.0,
                )));
        let closest_x = (self.local_position.x - self_half_width_x).max(
            local_sphere_center
                .x
                .min(self.local_position.x + self_half_width_x),
        );
        let closest_y = (self.local_position.y - self_half_height_y).max(
            local_sphere_center
                .y
                .min(self.local_position.y + self_half_height_y),
        );
        let closest_z = (self.local_position.z - self_half_depth_z).max(
            local_sphere_center
                .z
                .min(self.local_position.z + self_half_depth_z),
        );
        let offset: Vec4 = local_sphere_center - Vec4::new(closest_x, closest_y, closest_z, 1.0);
        if !(offset.length() < radius) || offset.length() == 0.0 {
            None
        } else {
            let impulse_strength = (radius - offset.length()) / offset.length();
            Option::Some(Vec3::new(offset.x, offset.y, offset.z) * impulse_strength)
        }
    }

    fn gjk(
        self_half_width_x: f32,
        self_half_height_y: f32,
        self_half_depth_z: f32,
        self_local_position: Vec3,
        half_width_x: f32,
        half_height_y: f32,
        half_depth_z: f32,
        local_position: Vec3,
        transform: &Mat4,
        other_transform: &Mat4,
    ) -> bool {
        let vertices: Vec<Vec3> = Collider::compute_vertices(
            self_half_width_x,
            self_half_height_y,
            self_half_depth_z,
            self_local_position,
            transform,
        );
        let other_vertices: Vec<Vec3> = Collider::compute_vertices(
            half_width_x,
            half_height_y,
            half_depth_z,
            local_position,
            other_transform,
        );

        let mut support: Vec3 =
            Collider::support(vertices.clone(), other_vertices.clone(), Vec3::unit_x());
        let mut simplex = Simplex::new();
        simplex.push_front(support);

        // New direction is towards the origin
        let mut direction: Vec3 = -support;

        let mut number_of_iterations: i32 = 0;
        let max_number_of_iterations: i32 = 5;
        while number_of_iterations < max_number_of_iterations {
            support = Collider::support(vertices.clone(), other_vertices.clone(), direction);

            if support.dot(direction) <= 0.0 {
                return false; // no collision
            }
            simplex.push_front(support);
            let next_simplex: SimplexDirectionCollision =
                Collider::next_simplex(simplex, direction);
            if next_simplex.is_colliding {
                return true;
            }
            simplex = next_simplex.simplex;
            direction = next_simplex.direction;
            number_of_iterations += 1;
        }
        false
    }

    fn find_furthest_point(direction: Vec3, vertices: &Vec<Vec3>) -> Vec3 {
        *vertices
            .iter()
            .max_by(|x, y| x.dot(direction).partial_cmp(&y.dot(direction)).unwrap())
            .unwrap()
    }

    fn support(collider_a: Vec<Vec3>, collider_b: Vec<Vec3>, direction: Vec3) -> Vec3 {
        Collider::find_furthest_point(direction, &collider_a)
            - Collider::find_furthest_point(-direction, &collider_b)
    }

    fn same_direction(direction: Vec3, point_to_origin: Vec3) -> bool {
        direction.dot(point_to_origin) > 0.0
    }

    fn compute_vertices(
        half_width_x: f32,
        half_height_y: f32,
        half_depth_z: f32,
        local_position: Vec3,
        transform: &Mat4,
    ) -> Vec<Vec3> {
        let mut edges: Vec<Vec3> = Vec::new();
        for (x, y, z) in iproduct!(
            vec![-half_width_x, half_width_x].into_iter(),
            vec![-half_height_y, half_height_y].into_iter(),
            vec![-half_depth_z, half_depth_z].into_iter()
        ) {
            let local_edge = Collider::compute_local_edge(local_position, x, y, z);
            let global_edge = Collider::compute_global_edge(transform.clone(), local_edge);
            edges.push(global_edge);
        }
        edges
    }

    fn compute_local_edge(local_position: Vec3, x: f32, y: f32, z: f32) -> Vec3 {
        local_position + x * Vec3::unit_x() + y * Vec3::unit_y() + z * Vec3::unit_z()
    }

    fn compute_global_edge(transform: Mat4, local_edge: Vec3) -> Vec3 {
        let global_edge: Vec4 =
            transform.mul_vec4(Vec4::new(local_edge.x, local_edge.y, local_edge.z, 1.0));
        Vec3::new(global_edge.x, global_edge.y, global_edge.z)
    }

    fn next_simplex(points: Simplex, direction: Vec3) -> SimplexDirectionCollision {
        match points.vertices.len() {
            2 => Collider::line(points),
            3 => Collider::triangle(points),
            4 => Collider::tetrahedron(points, direction),
            _ => SimplexDirectionCollision {
                simplex: points,
                direction,
                is_colliding: false,
            },
        }
    }

    fn line(points: Simplex) -> SimplexDirectionCollision {
        let a: Vec3 = points.vertices[0];
        let b: Vec3 = points.vertices[1];

        let ab: Vec3 = b - a;
        let ao: Vec3 = -a;

        let mut simplex: Simplex = Simplex::new();
        let direction: Vec3;
        if Collider::same_direction(ab, ao) {
            simplex = points;
            direction = ab.cross(ao).cross(ab);
        } else {
            simplex.vertices = vec![a];
            direction = ao;
        }
        SimplexDirectionCollision {
            simplex,
            direction,
            is_colliding: false,
        }
    }

    fn triangle(points: Simplex) -> SimplexDirectionCollision {
        let a: Vec3 = points.vertices[0];
        let b: Vec3 = points.vertices[1];
        let c: Vec3 = points.vertices[2];

        let ab: Vec3 = b - a;
        let ac: Vec3 = c - a;
        let ao: Vec3 = -a;

        let abc: Vec3 = ab.cross(ac);

        let mut simplex: Simplex = Simplex::new();

        if Collider::same_direction(abc.cross(ac), ao) {
            if Collider::same_direction(ac, ao) {
                simplex.vertices = vec![a, c];
                return SimplexDirectionCollision {
                    simplex,
                    direction: ac.cross(ao).cross(ac),
                    is_colliding: false,
                };
            } else {
                simplex.vertices = vec![a, b];
                return Collider::line(points);
            }
        } else {
            if Collider::same_direction(ab.cross(abc), ao) {
                simplex.vertices = vec![a, b];
                return Collider::line(points);
            } else {
                if Collider::same_direction(abc, ao) {
                    return SimplexDirectionCollision {
                        simplex: points,
                        direction: abc,
                        is_colliding: false,
                    };
                } else {
                    simplex.vertices = vec![a, c, b];
                    return SimplexDirectionCollision {
                        simplex,
                        direction: -abc,
                        is_colliding: false,
                    };
                }
            }
        }
    }

    fn tetrahedron(mut points: Simplex, direction: Vec3) -> SimplexDirectionCollision {
        let a: Vec3 = points.vertices[0];
        let b: Vec3 = points.vertices[1];
        let c: Vec3 = points.vertices[2];
        let d: Vec3 = points.vertices[3];

        let ab: Vec3 = b - a;
        let ac: Vec3 = c - a;
        let ad: Vec3 = d - a;
        let ao: Vec3 = -a;

        let abc: Vec3 = ab.cross(ac);
        let acd: Vec3 = ac.cross(ad);
        let adb: Vec3 = ad.cross(ab);

        if Collider::same_direction(abc, ao) {
            points.vertices = vec![a, b, c];
            return Collider::triangle(points);
        } else if Collider::same_direction(acd, ao) {
            points.vertices = vec![a, c, d];
            return Collider::triangle(points);
        } else if Collider::same_direction(adb, ao) {
            points.vertices = vec![a, d, b];
            return Collider::triangle(points);
        }
        SimplexDirectionCollision {
            simplex: points,
            direction,
            is_colliding: true,
        }
    }
}

struct SimplexDirectionCollision {
    simplex: Simplex,
    direction: Vec3,
    is_colliding: bool,
}

struct Simplex {
    vertices: Vec<Vec3>,
}

impl Simplex {
    pub fn new() -> Self {
        let vertices: Vec<Vec3> = Vec::new();
        Self { vertices }
    }

    pub fn push_front(&mut self, vertex: Vec3) -> &mut Simplex {
        self.vertices.insert(0, vertex);
        while self.vertices.len() > 4 {
            self.vertices.remove(4);
        }
        self
    }
}

pub fn collision_update(mut query: Query<(Entity, &Collider, &mut Transform)>) {
    let mut colliders = Vec::new();
    let mut other_colliders = Vec::new();
    for (entity, collider, transform) in query.iter_mut() {
        colliders.push((entity.clone(), collider.clone(), transform.compute_matrix()));
        other_colliders.push((entity.clone(), collider.clone(), transform.compute_matrix()));
    }
    let mut impulses = HashMap::new();
    for (entity, collider, collider_transform) in colliders.iter() {
        for (other_entity, other_collider, other_transform) in other_colliders.iter() {
            if !entity.id().eq(&other_entity.id()) {
                let impulse = collider
                    .detect_collision(other_collider, collider_transform, other_transform)
                    .unwrap_or(Vec3::zero());
                impulses
                    .entry(entity.id())
                    .or_insert(Vec3::zero())
                    .function(impulse);
            }
        }
        other_colliders.remove(0);
    }
    &impulses;
    for (entity, _collider, mut collider_transform) in query.iter_mut() {
        if impulses.contains_key(&entity.id()) {
            collider_transform.translation =
                collider_transform.translation - *impulses.get(&entity.id()).unwrap();
            collider_transform.translation;
        }
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
