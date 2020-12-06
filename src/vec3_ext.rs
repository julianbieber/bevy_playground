use bevy::math::Vec3;

pub trait Vec3DistanceExt {
    fn distance_sq(&self, other: &Vec3) -> f32;
}

impl Vec3DistanceExt for Vec3 {
    fn distance_sq(&self, other: &Vec3) -> f32 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        let z_diff = self.z - other.z;

        x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
    }
}
