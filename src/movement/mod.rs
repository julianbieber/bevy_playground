pub mod model;

use crate::movement::model::{Movable, MoveEvent, MovementReader, UnitRotation};
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<MoveEvent>()
            .init_resource::<MovementReader>()
            .add_system(movement_system.system());
    }
}

fn movement_system(
    mut movement_events: ResMut<MovementReader>,
    movement_reader: Res<Events<MoveEvent>>,
    mut units_query: Query<(&Movable, &mut Transform, &mut UnitRotation)>,
) {
    for movement in movement_events.reader.iter(&movement_reader) {
        let (_, mut transform, mut unit_rotation) = units_query.get_mut(movement.entity).unwrap();
        unit_rotation.rotation += movement.rotation_offset;
        unit_rotation.rotation.x = (&unit_rotation.rotation.x).rem_euclid(std::f32::consts::TAU);
        unit_rotation.rotation.y = (&unit_rotation.rotation.y).rem_euclid(std::f32::consts::TAU);
        unit_rotation.rotation.z = (&unit_rotation.rotation.z).rem_euclid(std::f32::consts::TAU);
        transform.rotation = Quat::from_rotation_ypr(
            unit_rotation.rotation.x,
            unit_rotation.rotation.y,
            unit_rotation.rotation.z,
        );

        let translation_offset = transform.rotation.mul_vec3(movement.translation_offset);
        transform.translation += translation_offset;
    }
}
