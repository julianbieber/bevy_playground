use crate::input::ReceivesInput;
use crate::physics::collider::*;

use bevy::prelude::*;

#[derive(Default)]
pub struct UnitRotation {
    pub rotation: Vec2,
}

/// this component indicates what entities should rotate
pub struct Movable;

#[derive(Default)]
pub struct MovementReader {
    reader: EventReader<MoveEvent>,
}

pub struct MoveEvent {
    pub rotation_offset: Vec2,
    pub translation_offset: Vec3,
    pub entity: Entity,
}

pub fn movement_system(
    mut movement_events: ResMut<MovementReader>,
    movement_reader: Res<Events<MoveEvent>>,
    mut units_query: Query<(&Movable, &mut Transform, &mut UnitRotation)>,
) {
    for movement in movement_events.reader.iter(&movement_reader) {
        let (_, mut transform, mut unit_rotation) = units_query.get_mut(movement.entity).unwrap();
        unit_rotation.rotation += movement.rotation_offset;

        transform.rotation =
            Quat::from_rotation_ypr(unit_rotation.rotation.x, unit_rotation.rotation.y, 0.0);

        let translation_offset = transform.rotation.mul_vec3(movement.translation_offset);
        transform.translation += translation_offset;
    }
}

pub fn player_setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 50.0)),
            ..Default::default()
        })
        .with(Collider {
            collider_shape: ColliderShapes::Cuboid {
                half_width_x: 0.25,
                half_height_y: 0.25,
                half_depth_z: 0.25,
            },
            local_position: Vec3::new(0.0, 0.0, 0.0),
        })
        .with(ReceivesInput)
        .with(Movable)
        .with(UnitRotation {
            ..Default::default()
        })
        .with_children(|parent| {
            let camera_position = Vec3::new(0.0, 1.0, 5.0);
            let camera_position_y = camera_position.y;
            let up = Vec3::unit_y();
            let camera_looking_point = -camera_position + 2.0 * camera_position_y * up;
            parent.spawn(Camera3dBundle {
                transform: Transform::from_translation(camera_position)
                    .looking_at(camera_looking_point, up),
                ..Default::default()
            });
        });
}
