use bevy::math::{Mat4, Vec3, Vec4};

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

pub trait Mat4Ext {
    fn transform_vec3(&self, other: Vec3) -> Vec3;
}

impl Mat4Ext for Mat4 {
    fn transform_vec3(&self, other: Vec3) -> Vec3 {
        let transformed_other_4 = self.mul_vec4(Vec4::new(other.x, other.y, other.z, 1.0f32));
        Vec3::new(
            transformed_other_4.x,
            transformed_other_4.y,
            transformed_other_4.z,
        )
    }
}
