mod body_of_water;
mod water_effect;
mod water_shaders;

use crate::water::body_of_water::{
    set_water_position, setup_water_layer, update_material_time, WaterMaterial,
};
use crate::water::water_effect::apply_water_raise;
use bevy::prelude::*;

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<WaterMaterial>()
            .add_startup_system(setup_water_layer.system())
            .add_system(update_material_time.system())
            .add_system(set_water_position.system())
            .add_system(apply_water_raise.system());
    }
}
