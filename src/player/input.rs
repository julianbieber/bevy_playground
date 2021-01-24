use crate::movement::model::{MoveEvent, UnitRotation};
use crate::player::model::ReceivesInput;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

const ROTATION_SPEED_X: f32 = 0.3f32;
const ROTATION_SPEED_Y: f32 = 0.3f32;

// m/s
const PLAYER_SPEED: f32 = 3.0f32;

pub fn publish_player_movements(
    mut mouse_events: EventReader<MouseMotion>,
    keys: Res<Input<KeyCode>>,
    mut movement_events: ResMut<Events<MoveEvent>>,
    mut input_receiver_query: Query<(Entity, &ReceivesInput, &UnitRotation)>,
    time: Res<Time>,
) {
    for (entity, _, unit_rotation) in input_receiver_query.iter_mut() {
        let mut frame_rotation = Vec3::zero();
        for event in mouse_events.iter() {
            let look = event.delta;
            frame_rotation.x -= (look.x).to_radians() / ROTATION_SPEED_X;
            frame_rotation.y -= (look.y).to_radians() / ROTATION_SPEED_Y;
        }
        frame_rotation *= time.delta_seconds();
        let rotation = cap_rotation(frame_rotation, &unit_rotation);

        let mut movement_before_rotation = Vec3::zero();
        if keys.pressed(KeyCode::W) {
            movement_before_rotation.z -= PLAYER_SPEED;
        }
        if keys.pressed(KeyCode::A) {
            movement_before_rotation.x -= PLAYER_SPEED;
        }
        if keys.pressed(KeyCode::D) {
            movement_before_rotation.x += PLAYER_SPEED;
        }
        if keys.pressed(KeyCode::S) {
            movement_before_rotation.z += PLAYER_SPEED;
        }
        if keys.pressed(KeyCode::Q) {
            movement_before_rotation.y += PLAYER_SPEED;
        }
        if keys.pressed(KeyCode::E) {
            movement_before_rotation.y -= PLAYER_SPEED;
        }
        movement_events.send(MoveEvent {
            rotation_offset: rotation,
            translation_offset: movement_before_rotation * time.delta_seconds(),
            entity,
        });
    }
}

fn cap_rotation(rotation: Vec3, current_rotation: &UnitRotation) -> Vec3 {
    let uncapped_rotation_y =
        (current_rotation.rotation.y + rotation.y).rem_euclid(std::f32::consts::TAU);

    let y = if !((uncapped_rotation_y - std::f32::consts::FRAC_PI_2).abs() < 0.2)
        && !((uncapped_rotation_y - 3.0 * std::f32::consts::FRAC_PI_2).abs() < 0.2)
    {
        rotation.y
    } else {
        0.0f32
    };
    Vec3::new(rotation.x, y, 0.0)
}
