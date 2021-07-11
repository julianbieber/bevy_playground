use crate::clouds::body_of_clouds::setup;
use bevy::prelude::*;

use self::body_of_clouds::IrrelevantMaterial;

pub(crate) mod body_of_clouds;

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<IrrelevantMaterial>()
            .add_startup_system(setup.system())
            .add_system(body_of_clouds::update_cloud_positon.system());
    }
}
