use crate::water::body_of_water::WaterMaterial;
use bevy::prelude::*;

pub struct WaterEffected;

pub fn apply_water_raise(
    material: Res<Assets<WaterMaterial>>,
    _: &WaterEffected,
    mut transform: Mut<Transform>,
    handle: &Handle<WaterMaterial>,
) {
    let position = transform.translation();
    for m in material.get(&handle).iter() {
        let water_level = (m.time * 0.1 + position.x()).sin() + (position.z() + 0.5).sin();
        if position.y() < water_level {
            transform.translate(Vec3::new(0.0, water_level - position.y(), 0.0));
        }
    }
}
