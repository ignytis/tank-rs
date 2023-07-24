use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::constants;
use crate::components::enemy::{Enemy, MovementMode};
use crate::geometry::{azimuth_to_quat_negative_z, get_closer_direction};

// These are the same as player's. Maybe drag them to constant module?
const HALF_ENEMY_SIZE: f32 = constants::TANK_DIMENSION / 2.;
const MOVEMENT_FACTOR_FORWARD: f32 = 3.0;
const ROTATION_FACTOR: f32 = 0.05;

/// Keeps moves enemy tank forward
pub fn move_enemies(
    mut query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
) {
    for (mut transform, mut enemy) in query.iter_mut() {
        match enemy.movement_mode {
            MovementMode::Move => {
                let v = transform.rotation * Vec3::Y * MOVEMENT_FACTOR_FORWARD;
                transform.translation += v;
            },
            MovementMode::Rotate(dest_rotation, direction, is_prev_angle_increased) => {
                let dir = if direction {1.} else {-1.};
                let angle_before = transform.rotation.angle_between(dest_rotation);
                enemy.azimuth += ROTATION_FACTOR * dir;
                if enemy.azimuth > 2.*PI {
                    enemy.azimuth -= 2.*PI;
                } else if enemy.azimuth < 2.*PI {
                    enemy.azimuth += 2.*PI;
                }
                transform.rotation = azimuth_to_quat_negative_z(enemy.azimuth);

                let angle_after = transform.rotation.angle_between(dest_rotation);
                let is_current_angle_increased = angle_after > angle_before;

                match is_prev_angle_increased {
                    None => {
                        enemy.start_rotate(dest_rotation, direction, Some(is_current_angle_increased));
                    },
                    Some(is_prev_angle_increased) => {
                        if is_prev_angle_increased != is_current_angle_increased { // previously the ancle in/de-creased, now it's otherwise
                            enemy.start_move();
                        }
                    },
                };
            },
        };
    }
}

/// Changes direction of enemy tank if it reached the screen edge
pub fn change_enemy_direction(
    mut query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = window.width() / -2.0 + HALF_ENEMY_SIZE;
    let x_max = window.width() / 2.0 - HALF_ENEMY_SIZE;
    let y_min = window.height() / -2.0 + HALF_ENEMY_SIZE;
    let y_max = window.height() / 2.0 - HALF_ENEMY_SIZE;

    for (mut transform, mut enemy) in query.iter_mut() {
        // Skip if tank is already rotating
        match enemy.movement_mode {
            MovementMode::Rotate(_, _, _) => continue,
            _ => {},
        };

        let mut translation = transform.translation;
        let is_min_x_reached = translation.x < x_min;
        let is_max_x_reached = translation.x > x_max;
        let is_min_y_reached = translation.y < y_min;
        let is_max_y_reached = translation.y > y_max;

        if !(is_min_x_reached || is_max_x_reached || is_min_y_reached || is_max_y_reached) {
            continue
        }
        
        let mut rng = rand::thread_rng();
        let rotation_angle_relative = rng.gen_range((0. as f32)..(PI as f32));
        let rotation_angle_shift = if translation.x < x_min {
            translation.x = x_min;
            0.
        } else if translation.x > x_max {
            translation.x = x_max;
            PI
        } else if translation.y < y_min {
            translation.y = y_min;
            PI*1.5
        } else if translation.y > y_max {
            translation.y = y_max;
            PI*0.5
        } else { 0. };// will bever happen
        transform.translation = translation;

        let target_quat = azimuth_to_quat_negative_z(rotation_angle_relative + rotation_angle_shift);
        let tank_quat = transform.rotation;
        let direction = get_closer_direction(tank_quat, target_quat);

        enemy.start_rotate(target_quat, direction, None);
    }
}