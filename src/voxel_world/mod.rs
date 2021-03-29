pub mod access;
pub mod boundaries;
pub mod chunk;
pub mod chunk_mesh;
pub mod collision;
pub mod generator;
mod lod;
mod mesh;
pub mod voxel;

pub fn distance_2_lod(distance: f32) -> i32 {
    if distance < 300.0 {
        1
    } else if distance < 600.0 {
        2
    } else if distance < 1200.0 {
        4
    } else if distance < 2400.0 {
        8
    } else if distance < 4800.0 {
        16
    } else {
        32
    }
}
