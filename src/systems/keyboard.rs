use bevy::prelude::*;

use crate::components::player::Player;
use crate::geometry::azimuth_to_quat_negative_z;

const MOVEMENT_FACTOR_BACKWARD: f32 = 1.3;
const MOVEMENT_FACTOR_FORWARD: f32 = 3.0;
const ROTATION_FACTOR: f32 = 0.05;

pub fn keyboard_events(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
) {
    let (mut player, mut transform, ) = match player_query.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if input.pressed(KeyCode::Up) {
        let v = transform.rotation * Vec3::Y * MOVEMENT_FACTOR_FORWARD;
        transform.translation += v;
    } else if input.pressed(KeyCode::Down) {
        let v = transform.rotation * Vec3::Y * MOVEMENT_FACTOR_BACKWARD * -1.0;
        transform.translation += v;
    } 

    let rotate_to = if input.pressed(KeyCode::Left) {
        Some(-1.0_f32 * ROTATION_FACTOR)
    } else if input.pressed(KeyCode::Right) {
        Some(1.0_f32 * ROTATION_FACTOR)
    } else {
        None
    };

    if let Some(r) = rotate_to {
        player.azimuth += r;
        transform.rotation = azimuth_to_quat_negative_z(player.azimuth);
    }
}