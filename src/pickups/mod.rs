use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    delayed_despawn::DelayedDespawns, particles::model::ParticleTypes, player::PlayerMarker,
};

pub struct Energy {
    pub amount: f32,
}

pub struct EnergyPlugin;

impl Plugin for EnergyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(EnergySpawnTimer {
            timer: Timer::new(Duration::from_millis(600), true),
        })
        .add_startup_system(setup_ui.system())
        .add_system(add_energy_to_players.system())
        .add_system(draw_in_energy.system())
        .add_system(regularily_spawn_energy.system())
        .add_system(display_energy.system());
    }
}

fn add_energy_to_players(
    mut commands: Commands,
    player_query: Query<(Entity, &PlayerMarker), Without<Energy>>,
) {
    for (entity, _) in player_query.iter() {
        commands.entity(entity).insert(Energy { amount: 0.0 });
    }
}

struct EnergySpawnTimer {
    timer: Timer,
}

fn regularily_spawn_energy(
    mut commands: Commands,
    storm_query: Query<(&Transform, &ParticleTypes)>,
    mut spawn_timer: ResMut<EnergySpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut despanws_res: ResMut<DelayedDespawns>,
) {
    if spawn_timer.timer.tick(time.delta()).just_finished() {
        let mut rng = SmallRng::from_entropy();
        for (storm_transform, particle_type) in storm_query.iter() {
            match particle_type {
                ParticleTypes::Explosion { .. } => {}
                ParticleTypes::HighStorm { depth } => {
                    let sphere = meshes.add(Mesh::from(shape::Icosphere {
                        radius: 1.0,
                        subdivisions: 5,
                    }));
                    let material = materials.add(StandardMaterial {
                        base_color: Color::rgb(0.6, 0.6, 0.7),
                        ..Default::default()
                    });
                    let entity = commands
                        .spawn_bundle(PbrBundle {
                            mesh: sphere,
                            material: material,
                            transform: Transform::from_translation(Vec3::new(
                                rng.gen_range(
                                    storm_transform.translation.x - depth
                                        ..storm_transform.translation.x + depth,
                                ),
                                rng.gen_range(0.0f32..100.0f32),
                                rng.gen_range(-100.0f32..100.0f32),
                            )),
                            ..Default::default()
                        })
                        .insert(Energy { amount: 10.0 })
                        .id();
                    despanws_res
                        .despawns
                        .push((Timer::from_seconds(100.0, false), entity));
                }
            }
        }
    }
}

fn draw_in_energy(
    mut commands: Commands,
    mut players_query: Query<(&mut Energy, &Transform), With<PlayerMarker>>,
    pickups_query: Query<(Entity, &Energy, &Transform), Without<PlayerMarker>>,
) {
    let mut despawn_entities: Vec<Entity> = Vec::new();
    for (mut player_energy, player_transform) in players_query.iter_mut() {
        for (entity, energy, energy_transform) in pickups_query.iter() {
            if despawn_entities.iter().find(|e| **e == entity).is_none()
                && player_transform
                    .translation
                    .distance_squared(energy_transform.translation)
                    < 25.0
            {
                player_energy.amount += energy.amount;
                despawn_entities.push(entity);
            }
        }
    }

    for e in despawn_entities.into_iter() {
        commands.entity(e).despawn();
    }
}

struct EnergyText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                String::new(),
                TextStyle {
                    font_size: 50.0,
                    color: Color::WHITE,
                    font,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(EnergyText);
}

fn display_energy(
    mut text_query: Query<&mut Text, With<EnergyText>>,
    energy_query: Query<&Energy, With<PlayerMarker>>,
) {
    for mut text in text_query.iter_mut() {
        if let Some(e) = energy_query.iter().next() {
            text.sections[0].value = format!("Energy: {}", e.amount);
        }
    }
}
