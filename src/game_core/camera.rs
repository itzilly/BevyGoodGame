use crate::game_core::player::Player;
use bevy::prelude::*;

// pub fn setup_camera() {
//
// }

pub fn follow_player_system(
    player_query: Query<(&Transform), With<Player>>,
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
) {
    if let Ok((camera, mut camera_transform)) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
