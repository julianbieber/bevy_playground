use bevy::prelude::*;
use physme::prelude3d::*;

use crate::water::body_of_water::WaterMaterial;
use crate::water::water_effect::WaterEffected;

pub fn world_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut water_materials: ResMut<Assets<WaterMaterial>>,
) {
    let cube = meshes.add(shape::Cube { size: 0.5 }.into());
    let bigcube = meshes.add(shape::Cube { size: 8.0 }.into());
    let material = water_materials.add(WaterMaterial { time: 0.0f32 });

    let material2 = water_materials.add(WaterMaterial { time: 0.0f32 });
    commands
        .spawn(PbrComponents {
            mesh: cube.clone_weak(),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Real(1.0))
                .with_status(Status::Semikinematic)
                .with_position(Vec3::new(0.0, 5.0, 0.0)),
        )
        .with(WaterEffected::new())
        .with(material)
        .with_children(|parent| {
            parent.spawn((Shape::from(Size3::new(1.0, 1.0, 1.0)),));
        })
        .spawn(PbrComponents {
            mesh: bigcube.clone_weak(),
            material: materials.add(Color::rgb(0.2, 0.8, 0.2).into()),
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: cube,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Real(1.0))
                .with_status(Status::Semikinematic)
                .with_position(Vec3::new(2.0, 5.0, 0.0)),
        )
        .with(WaterEffected::new())
        .with(material2)
        .with_children(|parent| {
            parent.spawn((Shape::from(Size3::new(1.0, 1.0, 1.0)),));
        })
        .spawn(PbrComponents {
            mesh: bigcube,
            material: materials.add(Color::rgb(0.2, 0.8, 0.2).into()),
            ..Default::default()
        })
        .with(
            RigidBody::new(Mass::Infinite)
                .with_status(Status::Static)
                .with_position(Vec3::new(0.0, -8.0, 0.0)),
        )
        .with_children(|parent| {
            parent.spawn((Shape::from(Size3::new(16.0, 16.0, 16.0)),));
        });
}
