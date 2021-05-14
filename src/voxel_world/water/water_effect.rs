use super::body_of_water::WaterMaterial;
use bevy::prelude::*;

pub struct WaterEffected {
    force: Vec2,
}

impl WaterEffected {
    pub fn reset(&mut self) {
        self.force = Vec2::ZERO;
    }

    pub fn add(&mut self, new: Vec2) {
        self.force += new;
    }
}

pub fn apply_water_raise(
    water_materials: Res<Assets<WaterMaterial>>,
    mut water_effected_query: Query<(&mut WaterEffected, &mut Transform)>,
    water_query: Query<(&Handle<WaterMaterial>,)>,
) {
    // TODO
}

fn calculate_water_height(t: f32, x: f32, _z: f32) -> f32 {
    0.0 // TODO
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

        let mut lowest_known_point = Vec2::ZERO;
        let mut lowest_known_height = std::f32::INFINITY;

        let mut highest_known_point = Vec2::ZERO;
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
