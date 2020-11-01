use crate::water::body_of_water::{WaterMaterial, WaterPosition};
use bevy::prelude::*;
use physme::dim3::RigidBody;

pub struct WaterEffected {
    force: Vec2,
}

impl WaterEffected {
    pub fn new() -> WaterEffected {
        WaterEffected {
            force: Vec2::zero(),
        }
    }

    pub fn reset(&mut self) {
        self.force = Vec2::zero();
    }

    pub fn add(&mut self, new: Vec2) {
        self.force += new;
    }
}

pub fn apply_water_raise(
    water_materials: Res<Assets<WaterMaterial>>,
    mut water_effected_query: Query<(&mut WaterEffected, &mut Transform, &mut RigidBody)>,
    water_query: Query<(&Handle<WaterMaterial>, &WaterPosition)>,
) {
    for (mut water_effected, mut transform, mut body) in water_effected_query.iter_mut() {
        for (water_material_handle, water_position) in water_query.iter() {
            if water_position.lies_within(transform.translation) {
                let water_material = water_materials.get(water_material_handle).unwrap();
                let position = transform.translation;
                let water_level =
                    calculate_water_height(water_material.time, position.x(), position.z());
                if position.y() < water_level {
                    let new_force = WaterSurrounding::from_point(
                        water_material.time,
                        position.x(),
                        position.z(),
                    )
                    .calculate_force();
                    water_effected.add(new_force);

                    transform.translation += Vec3::new(
                        water_effected.force.x(),
                        water_level - position.y(),
                        water_effected.force.y(),
                    );
                    body.position = transform.translation;
                } else {
                    water_effected.reset();
                }
            } else {
                water_effected.reset();
            }
        }
    }
}

fn calculate_water_height(t: f32, x: f32, z: f32) -> f32 {
    (t * 0.1 + x).sin() + (z + 0.5).sin()
}

#[derive(Debug)]
struct WaterSurrounding {
    lowest_point: Vec2,
    lowest_height: f32,
    highest_point: Vec2,
    highest_height: f32,
}

impl WaterSurrounding {
    fn from_point(t: f32, x: f32, z: f32) -> WaterSurrounding {
        let offset = 0.001f32;
        let points_around = vec![
            (x - offset, z - offset),
            (x - offset, z),
            (x - offset, z + offset),
            (x, z - offset),
            (x, z),
            (x, z + offset),
            (x + offset, z - offset),
            (x + offset, z),
            (x + offset, z + offset),
        ];

        let mut lowest_known_point = Vec2::zero();
        let mut lowest_known_height = std::f32::INFINITY;

        let mut highest_known_point = Vec2::zero();
        let mut highest_known_height = std::f32::NEG_INFINITY;

        for (around_x, around_z) in points_around.into_iter() {
            let around_height = calculate_water_height(t, around_x, around_z);
            if around_height < lowest_known_height {
                lowest_known_height = around_height;
                lowest_known_point = Vec2::new(around_x, around_z);
            }
            if around_height > highest_known_height {
                highest_known_height = around_height;
                highest_known_point = Vec2::new(around_x, around_z);
            }
        }

        WaterSurrounding {
            lowest_point: lowest_known_point,
            lowest_height: lowest_known_height,
            highest_point: highest_known_point,
            highest_height: highest_known_height,
        }
    }

    fn calculate_force(&self) -> Vec2 {
        (self.lowest_point - self.highest_point)
            * (self.highest_height - self.lowest_height)
            * 1000.0
    }
}
