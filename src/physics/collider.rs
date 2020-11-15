use bevy::prelude::Vec3;

enum ColliderShapes {
    Sphere {
        radius: f32,
    },
    Cube {
        half_size: f32,
    },
    Cuboid {
        half_width_x: f32,
        half_height_y: f32,
        half_depth_z: f32,
    },
}

struct Collider {
    colliderShape: ColliderShapes,
    local_position: Vec3,
}

impl Collider {
    fn detect_collision(&self, other: Collider) -> Option<Vec3> {
        None
    }
}
