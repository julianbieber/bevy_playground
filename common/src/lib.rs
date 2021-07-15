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

pub struct PlayerMarker;
pub struct PlayerPosition {
    pub position: Vec3,
}

#[derive(Clone, Copy)]
pub enum ParticleTypes {
    Explosion { radius: f32 },
    HighStorm { x: f32, y: f32, z: f32 },
}

impl ParticleTypes {
    pub fn within(&self, translation: Vec3, other: Vec3) -> bool {
        match self {
            ParticleTypes::Explosion { radius } => {
                translation.distance_squared(other) < radius * radius
            }
            ParticleTypes::HighStorm { x, y, z } => {
                other.x < translation.x + x
                    && other.x > translation.x - x
                    && other.y < translation.y + y
                    && other.y > translation.y - y
                    && other.z < translation.z + z
                    && other.z > translation.z - z
            }
        }
    }
}
