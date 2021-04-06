use std::{ops::AddAssign, time::Instant};

use super::voxel::{VoxelDirection, VoxelPosition};
use super::{chunk::VoxelChunk, voxel::VoxelFace};
use crate::voxel_world::voxel::{VoxelTypes, HALF_VOXEL_SIZE};
use bevy::prelude::Vec3;

pub fn combine_voxels(chunk: &VoxelChunk) -> Vec<VoxelFace> {
    if chunk.count == 0 {
        return Vec::new();
    }
    let mut faces = Vec::with_capacity((chunk.count / chunk.lod as usize) * 6);
    for (base_x, base_y, base_z) in iproduct!(
        (chunk.boundary.min[0]..chunk.boundary.max[0])
            .step_by(chunk.lod as usize)
            .into_iter(),
        (chunk.boundary.min[1]..chunk.boundary.max[1])
            .step_by(chunk.lod as usize)
            .into_iter(),
        (chunk.boundary.min[2]..chunk.boundary.max[2])
            .step_by(chunk.lod as usize)
            .into_iter()
    ) {
        top_face(base_x, base_y, base_z, chunk).map(|f| faces.push(f));
        bottom_face(base_x, base_y, base_z, chunk).map(|f| faces.push(f));
        left_face(base_x, base_y, base_z, chunk).map(|f| faces.push(f));
        right_face(base_x, base_y, base_z, chunk).map(|f| faces.push(f));
        front_face(base_x, base_y, base_z, chunk).map(|f| faces.push(f));
        back_face(base_x, base_y, base_z, chunk).map(|f| faces.push(f));
    }
    faces
}

fn top_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    if base_y + chunk.lod < chunk.boundary.max[1] {
        for (x, z) in iproduct!(
            (base_x..base_x + chunk.lod).into_iter(),
            (base_z..base_z + chunk.lod).into_iter()
        ) {
            let position = VoxelPosition {
                x,
                z,
                y: base_y + chunk.lod,
            };
            if chunk.get(&position).is_some() {
                return None;
            }
        }
    }

    let mut type_2_count: [(VoxelTypes, i32); 8] = [
        (VoxelTypes::BrownRock, 0),
        (VoxelTypes::DarkRock1, 0),
        (VoxelTypes::DarkRock2, 0),
        (VoxelTypes::GreyRock1, 0),
        (VoxelTypes::GreyRock2, 0),
        (VoxelTypes::Moss, 0),
        (VoxelTypes::GroundRock1, 0),
        (VoxelTypes::Snow, 0),
    ];
    for (x, z) in iproduct!(
        (base_x..base_x + chunk.lod).into_iter(),
        (base_z..base_z + chunk.lod).into_iter()
    ) {
        for y in (base_y..base_y + chunk.lod).rev() {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                for (inner, c) in type_2_count.iter_mut() {
                    if *inner == t {
                        c.add_assign(1);
                        break;
                    }
                }
                break;
            }
        }
    }
    type_2_count
        .iter()
        .max_by_key(|(_, c)| *c)
        .and_then(|(t, c)| {
            if *c > 0 {
                Some(construct_face(
                    t.clone(),
                    VoxelDirection::UP,
                    base_x,
                    base_y,
                    base_z,
                    chunk.lod,
                ))
            } else {
                None
            }
        })
}

fn bottom_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    if base_y - 1 >= chunk.boundary.min[1] {
        for (x, z) in iproduct!(
            (base_x..base_x + chunk.lod).into_iter(),
            (base_z..base_z + chunk.lod).into_iter()
        ) {
            let position = VoxelPosition {
                x,
                z,
                y: base_y - 1,
            };
            if chunk.get(&position).is_some() {
                return None;
            }
        }
    }

    let mut type_2_count: [(VoxelTypes, i32); 8] = [
        (VoxelTypes::BrownRock, 0),
        (VoxelTypes::DarkRock1, 0),
        (VoxelTypes::DarkRock2, 0),
        (VoxelTypes::GreyRock1, 0),
        (VoxelTypes::GreyRock2, 0),
        (VoxelTypes::Moss, 0),
        (VoxelTypes::GroundRock1, 0),
        (VoxelTypes::Snow, 0),
    ];
    for (x, z) in iproduct!(
        (base_x..base_x + chunk.lod).into_iter(),
        (base_z..base_z + chunk.lod).into_iter()
    ) {
        for y in (base_y..base_y + chunk.lod) {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                for (inner, c) in type_2_count.iter_mut() {
                    if *inner == t {
                        c.add_assign(1);
                        break;
                    }
                }
                break;
            }
        }
    }
    type_2_count
        .iter()
        .max_by_key(|(_, c)| *c)
        .and_then(|(t, c)| {
            if *c > 0 {
                Some(construct_face(
                    t.clone(),
                    VoxelDirection::DOWN,
                    base_x,
                    base_y,
                    base_z,
                    chunk.lod,
                ))
            } else {
                None
            }
        })
}

fn left_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    if base_x - 1 >= chunk.boundary.min[0] {
        for (y, z) in iproduct!(
            (base_y..base_y + chunk.lod).into_iter(),
            (base_z..base_z + chunk.lod).into_iter()
        ) {
            let position = VoxelPosition {
                z,
                y,
                x: base_x - 1,
            };
            if chunk.get(&position).is_some() {
                return None;
            }
        }
    }

    let mut type_2_count: [(VoxelTypes, i32); 8] = [
        (VoxelTypes::BrownRock, 0),
        (VoxelTypes::DarkRock1, 0),
        (VoxelTypes::DarkRock2, 0),
        (VoxelTypes::GreyRock1, 0),
        (VoxelTypes::GreyRock2, 0),
        (VoxelTypes::Moss, 0),
        (VoxelTypes::GroundRock1, 0),
        (VoxelTypes::Snow, 0),
    ];
    for (y, z) in iproduct!(
        (base_y..base_y + chunk.lod).into_iter(),
        (base_z..base_z + chunk.lod).into_iter()
    ) {
        for x in base_x..base_x + chunk.lod {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                for (inner, c) in type_2_count.iter_mut() {
                    if *inner == t {
                        c.add_assign(1);
                        break;
                    }
                }
                break;
            }
        }
    }
    type_2_count
        .iter()
        .max_by_key(|(_, c)| *c)
        .and_then(|(t, c)| {
            if *c > 0 {
                Some(construct_face(
                    t.clone(),
                    VoxelDirection::LEFT,
                    base_x,
                    base_y,
                    base_z,
                    chunk.lod,
                ))
            } else {
                None
            }
        })
}

fn right_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    if base_x + chunk.lod < chunk.boundary.max[0] {
        for (y, z) in iproduct!(
            (base_y..base_y + chunk.lod).into_iter(),
            (base_z..base_z + chunk.lod).into_iter()
        ) {
            let position = VoxelPosition {
                z,
                y,
                x: base_x + chunk.lod,
            };
            if chunk.get(&position).is_some() {
                return None;
            }
        }
    }

    let mut type_2_count: [(VoxelTypes, i32); 8] = [
        (VoxelTypes::BrownRock, 0),
        (VoxelTypes::DarkRock1, 0),
        (VoxelTypes::DarkRock2, 0),
        (VoxelTypes::GreyRock1, 0),
        (VoxelTypes::GreyRock2, 0),
        (VoxelTypes::Moss, 0),
        (VoxelTypes::GroundRock1, 0),
        (VoxelTypes::Snow, 0),
    ];
    for (y, z) in iproduct!(
        (base_y..base_y + chunk.lod).into_iter(),
        (base_z..base_z + chunk.lod).into_iter()
    ) {
        for x in (base_x..base_x + chunk.lod).rev() {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                for (inner, c) in type_2_count.iter_mut() {
                    if *inner == t {
                        c.add_assign(1);
                        break;
                    }
                }
                break;
            }
        }
    }
    type_2_count
        .iter()
        .max_by_key(|(_, c)| *c)
        .and_then(|(t, c)| {
            if *c > 0 {
                Some(construct_face(
                    t.clone(),
                    VoxelDirection::RIGHT,
                    base_x,
                    base_y,
                    base_z,
                    chunk.lod,
                ))
            } else {
                None
            }
        })
}

fn front_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    if base_z - 1 >= chunk.boundary.min[2] {
        for (x, y) in iproduct!(
            (base_x..base_x + chunk.lod).into_iter(),
            (base_y..base_y + chunk.lod).into_iter()
        ) {
            let position = VoxelPosition {
                x,
                y,
                z: base_z - 1,
            };
            if chunk.get(&position).is_some() {
                return None;
            }
        }
    }

    let mut type_2_count: [(VoxelTypes, i32); 8] = [
        (VoxelTypes::BrownRock, 0),
        (VoxelTypes::DarkRock1, 0),
        (VoxelTypes::DarkRock2, 0),
        (VoxelTypes::GreyRock1, 0),
        (VoxelTypes::GreyRock2, 0),
        (VoxelTypes::Moss, 0),
        (VoxelTypes::GroundRock1, 0),
        (VoxelTypes::Snow, 0),
    ];
    for (x, y) in iproduct!(
        (base_x..base_x + chunk.lod).into_iter(),
        (base_y..base_y + chunk.lod).into_iter()
    ) {
        for z in base_z..base_z + chunk.lod {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                for (inner, c) in type_2_count.iter_mut() {
                    if *inner == t {
                        c.add_assign(1);
                        break;
                    }
                }
                break;
            }
        }
    }
    type_2_count
        .iter()
        .max_by_key(|(_, c)| *c)
        .and_then(|(t, c)| {
            if *c > 0 {
                Some(construct_face(
                    t.clone(),
                    VoxelDirection::FRONT,
                    base_x,
                    base_y,
                    base_z,
                    chunk.lod,
                ))
            } else {
                None
            }
        })
}

fn back_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    if base_z + chunk.lod < chunk.boundary.max[2] {
        for (x, y) in iproduct!(
            (base_x..base_x + chunk.lod).into_iter(),
            (base_y..base_y + chunk.lod).into_iter()
        ) {
            let position = VoxelPosition {
                x,
                y,
                z: base_z + chunk.lod,
            };
            if chunk.get(&position).is_some() {
                return None;
            }
        }
    }

    let mut type_2_count: [(VoxelTypes, i32); 8] = [
        (VoxelTypes::BrownRock, 0),
        (VoxelTypes::DarkRock1, 0),
        (VoxelTypes::DarkRock2, 0),
        (VoxelTypes::GreyRock1, 0),
        (VoxelTypes::GreyRock2, 0),
        (VoxelTypes::Moss, 0),
        (VoxelTypes::GroundRock1, 0),
        (VoxelTypes::Snow, 0),
    ];
    for (x, y) in iproduct!(
        (base_x..base_x + chunk.lod).into_iter(),
        (base_y..base_y + chunk.lod).into_iter()
    ) {
        for z in (base_z..base_z + chunk.lod).rev() {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                for (inner, c) in type_2_count.iter_mut() {
                    if *inner == t {
                        c.add_assign(1);
                        break;
                    }
                }
                break;
            }
        }
    }
    type_2_count
        .iter()
        .max_by_key(|(_, c)| *c)
        .and_then(|(t, c)| {
            if *c > 0 {
                Some(construct_face(
                    t.clone(),
                    VoxelDirection::BACK,
                    base_x,
                    base_y,
                    base_z,
                    chunk.lod,
                ))
            } else {
                None
            }
        })
}

fn construct_face(
    typ: VoxelTypes,
    direction: VoxelDirection,
    base_x: i32,
    base_y: i32,
    base_z: i32,
    lod: i32,
) -> VoxelFace {
    let mut center = VoxelPosition {
        x: base_x,
        y: base_y,
        z: base_z,
    }
    .to_vec();

    if lod != 1 {
        center += Vec3::ONE * lod as f32 * HALF_VOXEL_SIZE;
    }

    VoxelFace::from_voxels(center, typ, direction, lod)
}
