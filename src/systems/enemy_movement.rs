use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::constants;
use crate::components::enemy::{Enemy, MovementMode};
use crate::geometry::quat_to_azimuth;

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
            MovementMode::Rotate(dest_rotation) => {
                if quat_to_azimuth(transform.rotation) <= quat_to_azimuth(dest_rotation) {
                    enemy.start_move();
                }

                transform.rotate_local_z(ROTATION_FACTOR);
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
        let mut translation = transform.translation;
        let is_min_x_reached = translation.x < x_min;
        let is_max_x_reached = translation.x > x_max;
        let is_min_y_reached = translation.y < y_min;
        let is_max_y_reached = translation.y > y_max;

        if !(is_min_x_reached || is_max_x_reached || is_min_y_reached || is_max_y_reached) {
            continue
        }
        
        let mut rng = rand::thread_rng();
        let random_angle = rng.gen_range((0. as f32)..(PI as f32));

        if translation.x < x_min {
            translation.x = x_min;
            enemy.start_rotate(Quat::from_rotation_z(random_angle));
        } else if translation.x > x_max {
            translation.x = x_max;
            enemy.start_rotate(Quat::from_rotation_z(random_angle + PI));
        } else if translation.y < y_min {
            translation.y = y_min;
            enemy.start_rotate(Quat::from_rotation_z(random_angle + PI*1.5));
        } else if translation.y > y_max {
            translation.y = y_max;
            enemy.start_rotate(Quat::from_rotation_z(random_angle + PI*0.5));
        }
        transform.translation = translation;
    }
}