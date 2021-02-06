use bevy::ecs::Query;
use bevy::prelude::*;
use cgmath::num_traits::Float;
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
                    let impulse_strength = (self_radius - offset.length()) / offset.length();
                    Option::Some(-Vec3::new(offset.x, offset.y, offset.z) * impulse_strength)
                }
            }
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
            ColliderShapes::Sphere { radius } => {
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
                let offset: Vec4 =
                    local_sphere_center - Vec4::new(closest_x, closest_y, closest_z, 1.0);
                if !(offset.length() < radius) || offset.length() == 0.0 {
                    None
                } else {
                    let impulse_strength = 0.5 * (radius - offset.length()) / offset.length();
                    Option::Some(Vec3::new(offset.x, offset.y, offset.z) * impulse_strength)
                }
            }
            ColliderShapes::Cuboid {
                half_width_x: _,
                half_height_y: _,
                half_depth_z: _,
            } => {
                Collider::GJK(self, other, transform, other_transform);
                None
            }
        }
    }

    fn find_furthest_point(direction: Vec3, vertices: &Vec<Vec3>) -> Vec3 {
        let mut maxPoint: Vec3 = Vec3::zero();
        let mut maxDistance: f32 = -f32::infinity();

        for vertex in vertices.iter() {
            let distance: f32 = vertex.dot(direction);
            if distance > maxDistance {
                maxDistance = distance;
                maxPoint = vertex.clone();
            }
        }
        maxPoint
    }

    fn compute_vertices(collider: &Collider, transform: &Mat4) -> Vec<Vec3> {
        let mut edges: Vec<Vec3> = Vec::new();
        match collider.collider_shape {
            ColliderShapes::Cuboid {
                half_width_x,
                half_height_y,
                half_depth_z,
            } => {
                let local_position = collider.local_position;
                for x in vec![-half_width_x, half_width_x] {
                    for y in vec![-half_height_y, half_height_y] {
                        for z in vec![-half_depth_z, half_depth_z] {
                            let local_edge = local_position
                                + x * Vec3::unit_x()
                                + y * Vec3::unit_y()
                                + z * Vec3::unit_z();
                            let global_edge: Vec4 = transform.mul_vec4(Vec4::new(
                                local_edge.x,
                                local_edge.y,
                                local_edge.z,
                                1.0,
                            ));
                            let g = Vec3::new(global_edge.x, global_edge.y, global_edge.z);
                            edges.push(g);
                        }
                    }
                }
            }
            ColliderShapes::Sphere { radius } => {}
        }
        edges
    }

    fn support(colliderA: Vec<Vec3>, colliderB: Vec<Vec3>, direction: Vec3) -> Vec3 {
        Collider::find_furthest_point(direction, &colliderA)
            - Collider::find_furthest_point(-direction, &colliderB)
    }

    fn GJK(
        collider: &Collider,
        other_collider: &Collider,
        transform: &Mat4,
        other_transform: &Mat4,
    ) -> bool {
        let vertices: Vec<Vec3> = Collider::compute_vertices(collider, transform);
        let other_vertices: Vec<Vec3> = Collider::compute_vertices(other_collider, other_transform);
        let mut support: Vec3 =
            Collider::support(vertices.clone(), other_vertices.clone(), Vec3::unit_x());
        let mut simplex = Simplex::new();
        simplex.push_front(support);
        // New direction is towards the origin
        let mut direction: Vec3 = -support;
        let mut number_of_iterations: i32 = 0;
        let max_number_of_iterations: i32 = 700;
        while number_of_iterations < max_number_of_iterations {
            support = Collider::support(vertices.clone(), other_vertices.clone(), direction);

            if support.dot(direction) <= 0.0 {
                return false; // no collision
            }
            simplex.push_front(support);
            let a: Help = Collider::next_simplex(simplex, direction);
            if a.boolean {
                return true;
            }
            simplex = a.simplex;
            direction = a.direction;
            number_of_iterations += 1;
        }
        false
    }

    fn same_direction(direction: Vec3, point_to_origin: Vec3) -> bool {
        direction.dot(point_to_origin) > 0.0
    }

    fn next_simplex(points: Simplex, direction: Vec3) -> Help {
        match points.number_of_vertices {
            2 => Collider::line(points, direction),
            3 => Collider::triangle(points, direction),
            4 => Collider::tetrahedron(points, direction),
            _ => Help {
                simplex: points,
                direction,
                boolean: false,
            },
        }
    }

    fn line(points: Simplex, mut direction: Vec3) -> Help {
        let a: Vec3 = points.vertices[0];
        let b: Vec3 = points.vertices[1];

        let ab: Vec3 = b - a;
        let ao: Vec3 = -a;

        let mut simplex: Simplex = Simplex::new();

        if Collider::same_direction(ab, ao) {
            simplex = points;
            direction = ab.cross(ao).cross(ab);
        } else {
            let mut p: Vec<Vec3> = Vec::new();
            p.push(a);
            simplex.number_of_vertices = p.len();
            simplex.vertices = p;
            direction = ao;
        }
        Help {
            simplex,
            direction,
            boolean: false,
        }
    }

    fn triangle(points: Simplex, mut direction: Vec3) -> Help {
        let a: Vec3 = points.vertices[0];
        let b: Vec3 = points.vertices[1];
        let c: Vec3 = points.vertices[2];

        let ab: Vec3 = b - a;
        let ac: Vec3 = c - a;
        let ao: Vec3 = -a;

        let abc: Vec3 = ab.cross(ac);

        let mut simplex: Simplex = Simplex::new();
        let mut s: Simplex = Simplex::new();
        let mut result: Help = Help {
            simplex: s,
            direction,
            boolean: false,
        };

        if Collider::same_direction(abc.cross(ac), ao) {
            if Collider::same_direction(ac, ao) {
                let mut p: Vec<Vec3> = Vec::new();
                p.push(a);
                p.push(c);
                simplex.number_of_vertices = p.len();
                simplex.vertices = p;
                direction = ac.cross(ao).cross(ac);
                result = Help {
                    simplex,
                    direction,
                    boolean: false,
                };
            } else {
                let mut p: Vec<Vec3> = Vec::new();
                p.push(a);
                p.push(b);
                simplex.number_of_vertices = p.len();
                simplex.vertices = p;
                return Collider::line(points, direction);
            }
        } else {
            if Collider::same_direction(ab.cross(abc), ao) {
                let mut p: Vec<Vec3> = Vec::new();
                p.push(a);
                p.push(b);
                simplex.number_of_vertices = p.len();
                simplex.vertices = p;
                return Collider::line(points, direction);
            } else {
                if Collider::same_direction(abc, ao) {
                    simplex = points;
                    direction = abc;
                    result = Help {
                        simplex,
                        direction,
                        boolean: false,
                    };
                } else {
                    let mut p: Vec<Vec3> = Vec::new();
                    p.push(a);
                    p.push(c);
                    p.push(b);
                    simplex.number_of_vertices = p.len();
                    simplex.vertices = p;
                    direction = -abc;
                    result = Help {
                        simplex,
                        direction,
                        boolean: false,
                    };
                }
            }
        }
        result
    }

    fn tetrahedron(mut points: Simplex, direction: Vec3) -> Help {
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
            let mut p: Vec<Vec3> = Vec::new();
            p.push(a);
            p.push(b);
            p.push(c);
            points.number_of_vertices = p.len();
            points.vertices = p;
            return Collider::triangle(points, direction);
        } else if Collider::same_direction(acd, ao) {
            let mut p: Vec<Vec3> = Vec::new();
            p.push(a);
            p.push(c);
            p.push(d);
            points.number_of_vertices = p.len();
            points.vertices = p;
            return Collider::triangle(points, direction);
        } else if Collider::same_direction(adb, ao) {
            let mut p: Vec<Vec3> = Vec::new();
            p.push(a);
            p.push(d);
            p.push(b);
            points.number_of_vertices = p.len();
            points.vertices = p;
            return Collider::triangle(points, direction);
        }
        Help {
            simplex: points,
            direction,
            boolean: true,
        }
    }
}
struct Help {
    simplex: Simplex,
    direction: Vec3,
    boolean: bool,
}

struct Simplex {
    vertices: Vec<Vec3>,
    number_of_vertices: usize,
}

impl Simplex {
    pub fn new() -> Self {
        let vertices: Vec<Vec3> = Vec::new();
        let number_of_vertices: usize = 0;
        Self {
            vertices,
            number_of_vertices,
        }
    }

    pub fn push_front(&mut self, vertex: Vec3) -> &mut Simplex {
        let mut v = Vec::new();
        v.push(vertex);
        let max_number_vertices: usize = 4;
        let vertices = min(max_number_vertices, self.number_of_vertices + 1);
        let mut i = 0;
        while v.len() < vertices {
            v.push(self.vertices.get(i).get_or_insert(&Vec3::zero()).clone());
            i = i + 1;
        }
        self.number_of_vertices = v.len();
        self.vertices = v;
        self
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
