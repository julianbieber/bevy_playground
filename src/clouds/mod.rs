use crate::clouds::body_of_clouds::setup;
use crate::clouds::body_of_clouds::CloudMaterial;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};
pub(crate) mod body_of_clouds;

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<CloudMaterial>()
            .add_startup_system(setup.system());
    }
}
