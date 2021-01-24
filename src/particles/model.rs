use bevy::prelude::*;

use std::time::Duration;

#[derive(Clone)]
pub enum ParticleTypes {
    Explosion { radius: f32 },
}

#[derive(Clone)]
pub struct ParticleDescription {
    pub typ: ParticleTypes,
    pub duration: Duration,
    pub particles: u32,
    pub position: Vec3,
}
