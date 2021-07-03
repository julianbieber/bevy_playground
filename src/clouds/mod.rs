use crate::clouds::body_of_clouds::setup;
use crate::clouds::body_of_clouds::CloudMaterial;
use bevy::prelude::*;

pub(crate) mod body_of_clouds;

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<CloudMaterial>()
            .add_startup_system(setup.system());
    }
}
