use bevy::prelude::*;

use std::time::Duration;

#[derive(Clone)]
pub enum ParticleTypes {
    Explosion { radius: f32 },
    HighStorm,
}

#[derive(Clone)]
pub struct ParticleDescription {
    pub typ: ParticleTypes,
    pub duration: Duration,
    pub particles: u32,
    pub position: Vec3,
}

impl ParticleDescription {
    pub fn explosion(
        radius: f32,
        duration: Duration,
        particles: u32,
        position: Vec3,
    ) -> ParticleDescription {
        ParticleDescription {
            typ: ParticleTypes::Explosion { radius },
            duration,
            particles,
            position,
        }
    }

    pub fn high_storm(duration: Duration, particles: u32, position: Vec3) -> ParticleDescription {
        ParticleDescription {
            typ: ParticleTypes::HighStorm,
            duration,
            particles,
            position,
        }
    }
}
