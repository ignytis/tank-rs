use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::player::Player;
use crate::constants;

const HALF_PLAYER_SIZE: f32 = constants::TANK_DIMENSION / 2.;

/// Keeps player tank in the game window
pub fn keep_player_in_window(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut player_transform = match player_query.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let window = window_query.get_single().unwrap();

    let x_min = window.width() / -2.0 + HALF_PLAYER_SIZE;
    let x_max = window.width() / 2.0 - HALF_PLAYER_SIZE;
    let y_min = window.height() / -2.0 + HALF_PLAYER_SIZE;
    let y_max = window.height() / 2.0 - HALF_PLAYER_SIZE;

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