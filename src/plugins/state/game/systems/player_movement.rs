use bevy::prelude::*;

use crate::plugins::state::game::components::player::Player;
use crate::constants;
use crate::geometry::azimuth_to_quat_negative_z;

const HALF_PLAYER_SIZE: f32 = constants::TANK_DIMENSION as f32 / 2.;


const MOVEMENT_FACTOR_BACKWARD: f32 = 1.3;
const MOVEMENT_FACTOR_FORWARD: f32 = 3.0;
const ROTATION_FACTOR: f32 = 0.05;

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
) {
    let (mut player, mut transform, ) = match player_query.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if input.pressed(KeyCode::ArrowUp) {
        let v = transform.rotation * Vec3::Y * MOVEMENT_FACTOR_FORWARD;
        transform.translation += v;
    } else if input.pressed(KeyCode::ArrowDown) {
        let v = transform.rotation * Vec3::Y * MOVEMENT_FACTOR_BACKWARD * -1.0;
        transform.translation += v;
    } 

    let rotate_to = if input.pressed(KeyCode::ArrowLeft) {
        Some(-1.0_f32 * ROTATION_FACTOR)
    } else if input.pressed(KeyCode::ArrowRight) {
        Some(1.0_f32 * ROTATION_FACTOR)
    } else {
        None
    };

    if let Some(r) = rotate_to {
        player.azimuth += r;
        transform.rotation = azimuth_to_quat_negative_z(player.azimuth);
    }
}

/// Keeps player tank in the game window
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = match player_query.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let x_min = constants::WINDOW_X_MIN + HALF_PLAYER_SIZE;
    let x_max = constants::WINDOW_X_MAX - constants::HUD_WIDTH - HALF_PLAYER_SIZE;
    let y_min = constants::WINDOW_Y_MIN + HALF_PLAYER_SIZE;
    let y_max = constants::WINDOW_Y_MAX - HALF_PLAYER_SIZE;

    let mut translation = player_transform.translation;
    if translation.x < x_min {
        translation.x = x_min;
    } else if translation.x > x_max {
        translation.x = x_max;
    }

    if translation.y < y_min {
        translation.y = y_min;
    } else if translation.y > y_max {
        translation.y = y_max;
    }

    player_transform.translation = translation;
}