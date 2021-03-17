use bevy::prelude::*;

#[derive(Default)]
pub struct UnitRotation {
    pub rotation: Vec3,
}

/// this component indicates what entities should rotate
pub struct Movable;

#[derive(Debug)]
pub struct MoveEvent {
    pub rotation_offset: Vec3,
    pub translation_offset: Vec3,
    pub entity: Entity,
    pub is_player: bool,
}
