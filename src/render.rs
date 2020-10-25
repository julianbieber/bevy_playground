use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn hide_far_away(
    mut objects: Query<(&mut Draw, &mut Transform)>,
    player: Query<(&PlayerPosition, &Camera)>,
) {
    for (p_transform, _) in player.iter() {
        for (mut o_draw, o_transform) in &mut objects.iter_mut() {
            let distance_sq = o_transform.translation.distance_sq(&p_transform.position);

            o_draw.is_visible = distance_sq < 100.0;
        }
    }
}

pub fn update_player_position(mut pp_query: Query<(&mut PlayerPosition, &Transform)>) {
    for (mut pp, t) in pp_query.iter_mut() {
        pp.position = t.translation;
    }
}

pub struct PlayerPosition {
    pub position: Vec3,
}

trait Vec3DistanceExt {
    fn distance_sq(&self, other: &Vec3) -> f32;
}

impl Vec3DistanceExt for Vec3 {
    fn distance_sq(&self, other: &Vec3) -> f32 {
        let x_diff = self.x() - other.x();
        let y_diff = self.z() - other.y();
        let z_diff = self.y() - other.z();

        x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
    }
}
