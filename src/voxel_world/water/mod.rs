mod body_of_water;
mod water;
mod water_effect;
mod water_mesh;
mod water_shaders;
mod water_source;

use self::water_effect::apply_water_raise;
use self::water_source::water_source;
use self::{
    body_of_water::{
        internal_water_physics, setup_water_object, update_material_time, update_water_mesh,
        WaterMaterial,
    },
    water::WaterOperations,
};
use bevy::{core::FixedTimestep, prelude::*};

pub struct WaterPlugin;
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;


impl Plugin for WaterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<WaterMaterial>()
            .insert_resource(WaterOperations::new())
            .add_startup_system(setup_water_object.system())
            .add_system(update_material_time.system())
            .add_system(apply_water_raise.system())
            .add_system(update_water_mesh.system())
            .add_stage_after(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(
                        FixedTimestep::step(0.1),
                    )
                    .with_system(internal_water_physics.system()),
            )
            .add_system(water_source.system());
    }
}
