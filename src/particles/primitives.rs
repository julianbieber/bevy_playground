use bevy::{math::Mat3, prelude::Vec3};

pub fn cube_vertices(length: f32) -> [[f32; 3]; 8] {
    let sp = bevy::render::mesh::shape::Box::new(length, length, length);
    [
        [sp.min_x, sp.min_y, sp.max_z],
        [sp.max_x, sp.min_y, sp.max_z],
        [sp.max_x, sp.max_y, sp.max_z],
        [sp.min_x, sp.max_y, sp.max_z],
        [sp.min_x, sp.min_y, sp.min_z],
        [sp.max_x, sp.min_y, sp.min_z],
        [sp.max_x, sp.max_y, sp.min_z],
        [sp.min_x, sp.max_y, sp.min_z],
    ]
}

pub fn cube_indices(i: u32) -> Vec<u32> {
    vec![
        i * 8 + 0,
        i * 8 + 1,
        i * 8 + 2,
        i * 8 + 2,
        i * 8 + 3,
        i * 8 + 0,
        i * 8 + 1,
        i * 8 + 5,
        i * 8 + 6,
        i * 8 + 6,
        i * 8 + 2,
        i * 8 + 1,
        i * 8 + 7,
        i * 8 + 6,
        i * 8 + 5,
        i * 8 + 5,
        i * 8 + 4,
        i * 8 + 7,
        i * 8 + 4,
        i * 8 + 0,
        i * 8 + 3,
        i * 8 + 3,
        i * 8 + 7,
        i * 8 + 4,
        i * 8 + 4,
        i * 8 + 5,
        i * 8 + 1,
        i * 8 + 1,
        i * 8 + 0,
        i * 8 + 4,
        i * 8 + 3,
        i * 8 + 2,
        i * 8 + 6,
        i * 8 + 6,
        i * 8 + 7,
        i * 8 + 3,
    ]
}

pub fn triangle_vertices(length: f32, offset: Vec3, rotation: Mat3) -> [[f32; 3]; 3] {
    let x = Vec3::new(0.0, length, 0.0) + offset;
    let y = rotation * Vec3::new(length, 0.0, 0.0) + offset;
    let z = rotation * Vec3::new(0.0, 0.0, length) + offset;

    [[x.x, x.y, x.z], [y.x, y.y, y.z], [z.x, z.y, z.z]]
}

pub fn triangle_indices(i: u32) -> Vec<u32> {
    vec![i * 3, i * 3 + 1, i * 3 + 2]
}
