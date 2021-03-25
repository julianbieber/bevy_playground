use std::ops::AddAssign;

use ahash::AHashMap;

use super::voxel::{VoxelDirection, VoxelPosition};
use super::{chunk::VoxelChunk, voxel::VoxelFace};
use crate::voxel_world::voxel::{Voxel, VoxelTypes, HALF_VOXEL_SIZE, VOXEL_SIZE};
use bevy::prelude::Vec3;

pub fn combine_voxels(chunk: &VoxelChunk) -> Vec<VoxelFace> {
    let mut faces = Vec::with_capacity(chunk.count / chunk.lod as usize * 6);
    for (base_x, base_y, base_z) in iproduct!(
        (chunk.boundary.min[0]..chunk.boundary.max[0] + 1)
            .step_by(chunk.lod as usize)
            .into_iter(),
        (chunk.boundary.min[1]..chunk.boundary.max[1] + 1)
            .step_by(chunk.lod as usize)
            .into_iter(),
        (chunk.boundary.min[2]..chunk.boundary.max[2] + 1)
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
    let mut position_2_type = Vec::with_capacity((chunk.lod * chunk.lod) as usize);
    for (x, z) in iproduct!(
        (base_x..base_x + chunk.lod).into_iter(),
        (base_z..base_z + chunk.lod).into_iter()
    ) {
        let projected = VoxelPosition {
            x,
            z,
            y: base_y + chunk.lod - 1,
        };
        for y in (base_y..base_y + chunk.lod).rev() {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                position_2_type.push((projected, Some(t.clone())));
                break;
            } else {
                if y == base_y {
                    position_2_type.push((projected, None));
                }
            }
        }
    }

    construct_face(
        &position_2_type,
        VoxelDirection::UP,
        base_x,
        base_y,
        base_z,
        chunk,
    )
}

fn bottom_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    let mut position_2_type = Vec::with_capacity((chunk.lod * chunk.lod) as usize);
    for (x, z) in iproduct!(
        (base_x..base_x + chunk.lod).into_iter(),
        (base_z..base_z + chunk.lod).into_iter()
    ) {
        let projected = VoxelPosition { x, z, y: base_y };
        for y in (base_y..base_y + chunk.lod) {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                position_2_type.push((projected, Some(t.clone())));
                break;
            } else {
                if y == base_y + chunk.lod - 1 {
                    position_2_type.push((projected, None));
                }
            }
        }
    }
    construct_face(
        &position_2_type,
        VoxelDirection::DOWN,
        base_x,
        base_y,
        base_z,
        chunk,
    )
}

fn left_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    let mut position_2_type = Vec::with_capacity((chunk.lod * chunk.lod) as usize);
    for (y, z) in iproduct!(
        (base_y..base_y + chunk.lod).into_iter(),
        (base_z..base_z + chunk.lod).into_iter()
    ) {
        let projected = VoxelPosition { x: base_x, z, y };
        for x in base_x..base_x + chunk.lod {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                position_2_type.push((projected, Some(t.clone())));
                break;
            } else {
                if x == base_x + chunk.lod - 1 {
                    position_2_type.push((projected, None));
                }
            }
        }
    }
    construct_face(
        &position_2_type,
        VoxelDirection::LEFT,
        base_x,
        base_y,
        base_z,
        chunk,
    )
}

fn right_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    let mut position_2_type = Vec::with_capacity((chunk.lod * chunk.lod) as usize);
    for (y, z) in iproduct!(
        (base_y..base_y + chunk.lod).into_iter(),
        (base_z..base_z + chunk.lod).into_iter()
    ) {
        let projected = VoxelPosition {
            x: base_x + chunk.lod - 1,
            z,
            y,
        };
        for x in (base_x..base_x + chunk.lod).rev() {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                position_2_type.push((projected, Some(t.clone())));
                break;
            } else {
                if x == base_x {
                    position_2_type.push((projected, None));
                }
            }
        }
    }
    construct_face(
        &position_2_type,
        VoxelDirection::RIGHT,
        base_x,
        base_y,
        base_z,
        chunk,
    )
}

fn front_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    let mut position_2_type = Vec::with_capacity((chunk.lod * chunk.lod) as usize);
    for (x, y) in iproduct!(
        (base_x..base_x + chunk.lod).into_iter(),
        (base_y..base_y + chunk.lod).into_iter()
    ) {
        let projected = VoxelPosition { x, z: base_z, y };
        for z in base_z..base_z + chunk.lod {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                position_2_type.push((projected, Some(t.clone())));
                break;
            } else {
                if z == base_z + chunk.lod - 1 {
                    position_2_type.push((projected, None));
                }
            }
        }
    }

    construct_face(
        &position_2_type,
        VoxelDirection::FRONT,
        base_x,
        base_y,
        base_z,
        chunk,
    )
}

fn back_face(base_x: i32, base_y: i32, base_z: i32, chunk: &VoxelChunk) -> Option<VoxelFace> {
    let mut position_2_type = Vec::with_capacity((chunk.lod * chunk.lod) as usize);
    for (x, y) in iproduct!(
        (base_x..base_x + chunk.lod).into_iter(),
        (base_y..base_y + chunk.lod).into_iter()
    ) {
        let projected = VoxelPosition {
            x,
            z: base_z + chunk.lod - 1,
            y,
        };
        for z in (base_z..base_z + chunk.lod).rev() {
            let p = VoxelPosition { x, y, z };
            if let Some(t) = chunk.get(&p) {
                position_2_type.push((projected, Some(t.clone())));
                break;
            } else {
                if z == base_z {
                    position_2_type.push((projected, None));
                }
            }
        }
    }
    construct_face(
        &position_2_type,
        VoxelDirection::BACK,
        base_x,
        base_y,
        base_z,
        chunk,
    )
}

fn construct_face(
    position_2_type: &Vec<(VoxelPosition, Option<VoxelTypes>)>,
    direction: VoxelDirection,
    base_x: i32,
    base_y: i32,
    base_z: i32,
    chunk: &VoxelChunk,
) -> Option<VoxelFace> {
    if position_2_type.iter().filter(|(_, t)| t.is_some()).count() == 0
        || position_2_type
            .iter()
            .filter(|(p, _)| chunk.get(&p.in_direction(direction)).is_some())
            .count()
            == position_2_type.len()
    {
        None
    } else {
        let mut type_2_count = AHashMap::new();
        for (_, t_o) in position_2_type.iter() {
            if let Some(t) = t_o {
                type_2_count.entry((*t).clone()).or_insert(0).add_assign(1);
            }
        }
        let typ = type_2_count
            .iter()
            .max_by(|(_, c1), (_, c2)| (*c1).cmp(*c2))
            .unwrap()
            .0
            .clone();
        let mut center = VoxelPosition {
            x: base_x,
            y: base_y,
            z: base_z,
        }
        .to_vec();

        if chunk.lod != 1 {
            center += Vec3::ONE * chunk.lod as f32 * HALF_VOXEL_SIZE;
        }

        Some(VoxelFace::from_voxels(center, typ, direction, chunk.lod))
    }
}
