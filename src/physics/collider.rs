use bevy::prelude::*;
use cgmath::num_traits::Float;

use bevy::math::Vec4Swizzles;
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
                let collider: Vec<Vec3> = Collider::compute_vertices(
                    self_half_width_x,
                    self_half_height_y,
                    self_half_depth_z,
                    self.local_position,
                    transform,
                );
                let other_collider: Vec<Vec3> = Collider::compute_vertices(
                    half_width_x,
                    half_height_y,
                    half_depth_z,
                    other.local_position,
                    other_transform,
                );
                if let Some(collision) = Collider::gjk(&collider, &other_collider) {
                    let impulse = Collider::epa(collision.simplex, collider, other_collider);
                    return Option::Some(impulse.penetration_depth * impulse.normal);
                }
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

    fn gjk(vertices: &Vec<Vec3>, other_vertices: &Vec<Vec3>) -> Option<SimplexDirectionCollision> {
        let mut support: Vec3 =
            Collider::support(vertices.to_owned(), other_vertices.to_owned(), Vec3::X);
        let mut simplex = Simplex::new();
        simplex.push_front(support);

        // New direction is towards the origin
        let mut direction: Vec3 = -support;

        let mut number_of_iterations: i32 = 0;
        let max_number_of_iterations: i32 = 5;
        while number_of_iterations < max_number_of_iterations {
            support = Collider::support(vertices.clone(), other_vertices.clone(), direction);

            if support.dot(direction) <= 0.0 {
                return None; // no collision
            }
            simplex.push_front(support);
            let next_simplex: SimplexDirectionCollision =
                Collider::next_simplex(simplex, direction);
            if next_simplex.is_colliding {
                return Option::Some(next_simplex);
            }
            simplex = next_simplex.simplex;
            direction = next_simplex.direction;
            number_of_iterations += 1;
        }
        None
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
        local_position + x * Vec3::X + y * Vec3::Y + z * Vec3::Z
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

    fn epa(simplex: Simplex, colliderA: Vec<Vec3>, colliderB: Vec<Vec3>) -> CollisionPoints {
        let mut polytop: Polytop = Polytop::new(simplex.vertices.clone());

        let mut min_normal: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut min_distance: f32 = f32::infinity();

        let mut normals_min_triangle = Collider::get_face_normals(&polytop.polytop, &polytop.faces);

        let mut number_of_iterations = 0;
        let max_number_of_iterations = 20;

        while number_of_iterations < max_number_of_iterations && min_distance == f32::infinity() {
            number_of_iterations += 1;

            min_normal =
                normals_min_triangle.normals[normals_min_triangle.min_triangle as usize].xyz();
            min_distance = normals_min_triangle.w(normals_min_triangle.min_triangle as usize);

            let support: Vec3 =
                Collider::support((*colliderA).to_owned(), (*colliderB).to_owned(), min_normal);
            let s_distance: f32 = min_normal.dot(support);

            if f32::abs(s_distance - min_distance) > 0.01f32 {
                min_distance = f32::infinity();
            }

            let mut unique_edges: Vec<Vec<usize>> = Vec::new();

            for mut i in 0..(normals_min_triangle.normals.len()) {
                let mut number_removed_normals = 0;

                if i < normals_min_triangle.normals.len() - number_removed_normals {
                    if Collider::same_direction(
                        Vec3::from(normals_min_triangle.normals[i]),
                        support,
                    ) {
                        number_removed_normals += 1;
                        let f: i64 = (i * 3) as i64;
                        Collider::add_if_unique_edge(&mut unique_edges, &polytop.faces, f, f + 1);
                        Collider::add_if_unique_edge(
                            &mut unique_edges,
                            &polytop.faces,
                            f + 1,
                            f + 2,
                        );
                        Collider::add_if_unique_edge(&mut unique_edges, &polytop.faces, f + 2, f);

                        polytop.remove_face(f as usize);

                        normals_min_triangle.remove_normal(i);

                        i -= 1;
                    }
                }
            }
            let mut new_faces: Vec<usize> = Vec::new();
            for x in unique_edges.iter() {
                new_faces.push(x[0]);
                new_faces.push(x[1]);
                new_faces.push(polytop.polytop.len());
            }

            polytop.polytop.push(support);

            let mut new_normals_min_triangle =
                Collider::get_face_normals(&polytop.polytop, &new_faces);
            let mut old_min_distance: f32 = f32::infinity();

            old_min_distance = normals_min_triangle.find_minimal_distance(old_min_distance);

            if new_normals_min_triangle.w(new_normals_min_triangle.min_triangle as usize)
                < old_min_distance
            {
                new_normals_min_triangle.min_triangle = new_normals_min_triangle.min_triangle
                    + normals_min_triangle.normals.len() as u64;
            }

            polytop.faces.append(&mut new_faces.clone());
            normals_min_triangle
                .normals
                .append(&mut new_normals_min_triangle.normals);
        }

        min_distance = Collider::zero_if_infinity(min_distance);

        CollisionPoints {
            normal: min_normal,
            penetration_depth: min_distance + 0.001f32,
            has_collision: true,
        }
    }

    fn zero_if_infinity(u: f32) -> f32 {
        if u.is_infinite() {
            return 0.0f32;
        }
        u
    }

    fn get_face_normals(polytope: &Vec<Vec3>, faces: &Vec<usize>) -> FaceNormalsMinTriangle {
        let mut normals: Vec<Vec4> = Vec::new();
        let mut min_triangle: u64 = 0;
        let mut min_distance: f32 = f32::infinity();
        for i in (0..faces.len()).step_by(3) {
            let a: Vec3 = polytope[faces[i]];
            let b: Vec3 = polytope[faces[i + 1]];
            let c: Vec3 = polytope[faces[i + 2]];

            let mut normal: Vec3 = (b - a).cross(c - a).normalize();
            let mut distance: f32 = normal.dot(a);

            if distance < 0.0f32 {
                normal *= -1.0f32;
                distance *= -1.0f32;
            }
            normals.push(Vec4::new(normal.x, normal.y, normal.z, distance));
            if distance < min_distance {
                min_triangle = (i / 3) as u64;
                min_distance = distance;
            }
        }

        FaceNormalsMinTriangle {
            normals,
            min_triangle,
        }
    }

    fn add_if_unique_edge(edges: &mut Vec<Vec<usize>>, faces: &Vec<usize>, a: i64, b: i64) -> () {
        //use different type, s.t. it
        let reverse = edges
            .iter()
            .position(|x| *x == vec![faces[b as usize], faces[a as usize]]);
        if reverse.is_some() {
            edges.remove(reverse.unwrap());
        } else {
            edges.push(vec![faces[a as usize], faces[b as usize]]);
        }
    }
}

struct Polytop {
    polytop: Vec<Vec3>,
    faces: Vec<usize>,
}
impl Polytop {
    pub fn new(simplex_vertices: Vec<Vec3>) -> Self {
        let polytop: Vec<Vec3> = simplex_vertices;
        let faces: Vec<usize> = vec![0, 1, 2, 0, 3, 1, 0, 2, 3, 1, 3, 2];
        Self { polytop, faces }
    }

    pub fn remove_face(&mut self, f: usize) -> &mut Polytop {
        self.faces[(f + 2) as usize] = *self.faces.last().unwrap();
        self.faces.pop();
        self.faces[(f + 1) as usize] = *self.faces.last().unwrap();
        self.faces.pop();
        self.faces[f as usize] = *self.faces.last().unwrap();
        self.faces.pop();
        self
    }
}

struct CollisionPoints {
    normal: Vec3,
    penetration_depth: f32,
    has_collision: bool,
}

struct FaceNormalsMinTriangle {
    normals: Vec<Vec4>,
    min_triangle: u64,
}

impl FaceNormalsMinTriangle {
    pub fn remove_normal(&mut self, i: usize) -> &mut FaceNormalsMinTriangle {
        self.normals[i] = *self.normals.last().unwrap();
        self.normals.pop();
        self
    }

    pub fn w(&self, i: usize) -> f32 {
        self.normals[i].w
    }

    pub fn find_minimal_distance(&mut self, mut current_minimum: f32) -> f32 {
        for i in 0..self.normals.len() {
            if self.w(i) < current_minimum {
                current_minimum = self.w(i);
                self.min_triangle = i as u64;
            }
        }
        current_minimum
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
                    .unwrap_or(Vec3::ZERO);
                impulses
                    .entry(entity.id())
                    .or_insert(Vec3::ZERO)
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
