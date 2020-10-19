use crate::water::body_of_water::{WaterMaterial, WaterPosition};
use bevy::prelude::*;

pub struct WaterEffected;

pub fn apply_water_raise(
    water_materials: Res<Assets<WaterMaterial>>,
    mut water_effected_query: Query<(&mut WaterEffected, &mut Transform)>,
    mut water_query: Query<(&Handle<WaterMaterial>, &WaterPosition)>,
) {
    for (_water_effected, mut transform) in &mut water_effected_query.iter() {
        for (water_material_handle, water_position) in &mut water_query.iter() {
            if water_position.lies_within(transform.translation()) {
                let water_material = water_materials.get(water_material_handle).unwrap();
                let position = transform.translation();
                let water_level = calculate_water_height(water_material.time, position.x(), position.z());
                if position.y() < water_level {
                    transform.translate(Vec3::new(0.0, water_level - position.y(), 0.0));
                }
            }

        }
    }
}

fn calculate_water_height(t: f32, x: f32, z: f32) -> f32 {
    (t * 0.1 + x).sin() + (z + 0.5).sin()
}
