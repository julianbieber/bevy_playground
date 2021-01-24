use bevy::{
    prelude::*,
};

use std::time::Duration;

#[derive(Clone)]
pub struct Explosion {
    pub duration: Duration,
    pub radius: f32,
    pub particles: u32,
    pub position: Vec3,
}
