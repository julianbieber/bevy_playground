use super::vec3_ext::*;
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
