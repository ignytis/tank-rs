use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::player::Player;

const HALF_PLAYER_SIZE_X: f32 = 352.0 / 2.0;
const HALF_PLAYER_SIZE_Y: f32 = 354.0 / 2.0;

/// Keeps player's tank in the game window
pub fn keep_player_in_window(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = window.width() / -2.0 + HALF_PLAYER_SIZE_X * player_transform.scale.x;
        let x_max = window.width() / 2.0 - HALF_PLAYER_SIZE_X * player_transform.scale.x;
        let y_min = window.height() / -2.0 + HALF_PLAYER_SIZE_Y * player_transform.scale.y;
        let y_max = window.height() / 2.0 - HALF_PLAYER_SIZE_Y * player_transform.scale.y;

        let mut translation = player_transform.translation;
        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}