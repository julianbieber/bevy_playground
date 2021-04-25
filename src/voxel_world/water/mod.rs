mod body_of_water;
mod water_effect;
mod water_shaders;

use self::body_of_water::{
    setup_water_object, update_material_time, update_water_mesh, WaterMaterial,
};
use self::water_effect::apply_water_raise;
use bevy::prelude::*;

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<WaterMaterial>()
            .add_startup_system(setup_water_object.system())
            .add_system(update_material_time.system())
            .add_system(apply_water_raise.system())
            .add_system(update_water_mesh.system());
    }
}
