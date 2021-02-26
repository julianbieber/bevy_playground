mod effects;
mod evaluation;
mod internal_model;
pub mod model;

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::physics::collider::{Collider, ColliderShapes};
use crate::voxel_world::generator::VoxelWorld;
use flume::{unbounded, Sender};
use rand::prelude::*;

use self::{
    effects::{erosion, move_floating_voxels},
    evaluation::{
        evaluate_delayed_transformations, update_world_event_reader, update_world_from_channel,
    },
    internal_model::FreeFloatingVoxel,
    model::{DelayedWorldTransformations, WorldUpdateEvent, WorldUpdateResult},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (tx, rx) = unbounded::<WorldUpdateResult>();
        app.insert_resource(tx)
            .insert_resource(rx)
            .insert_resource(DelayedWorldTransformations {
                transformations: Vec::new(),
            })
            .add_event::<WorldUpdateEvent>()
            .add_system(update_world_from_channel.system())
            .add_system(update_world_event_reader.system())
            .add_system(erosion.system())
            .add_system(evaluate_delayed_transformations.system())
            .add_system(move_floating_voxels.system())
            .add_startup_system(world_setup.system());
    }
}

fn world_setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pool: ResMut<AsyncComputeTaskPool>,
    tx: Res<Sender<WorldUpdateResult>>,
) {
    let w = VoxelWorld::generate(150, 150, SmallRng::from_entropy());
    w.add_to_world(pool, tx);

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 100.0, 4.0)),
        ..Default::default()
    });

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 0.5 }));
    let cube_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.0, 1.0, 0.0),
        ..Default::default()
    });
    commands
        // parent cube
        .spawn(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 22.0, 0.1)),
            ..Default::default()
        })
        .with(Collider {
            collider_shape: ColliderShapes::Cuboid {
                half_width_x: 0.25,
                half_height_y: 0.25,
                half_depth_z: 0.25,
            },
            local_position: Vec3::new(0.0, 0.0, 0.0),
        });
}
