use std::borrow::BorrowMut;
use std::collections::LinkedList;
use std::f32::consts::{PI, FRAC_PI_2, TAU};

use bevy::prelude::*;
use bevy::utils::HashMap;
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
                if enemy.azimuth > TAU {
                    enemy.azimuth -= TAU;
                } else if enemy.azimuth < TAU {
                    enemy.azimuth += TAU;
                }
                transform.rotation = azimuth_to_quat_negative_z(enemy.azimuth);

                let angle_after = transform.rotation.angle_between(dest_rotation);
                let is_current_angle_increased = angle_after > angle_before;

                match is_prev_angle_increased {
                    None => {
                        enemy.continue_rotate(dest_rotation, is_current_angle_increased);
                    },
                    Some(is_prev_angle_increased) => {
                        if is_prev_angle_increased != is_current_angle_increased { // previously the angle in/de-creased, now it's otherwise
                            enemy.start_move();
                        }
                    },
                };
            },
        };
    }
}

/// Changes direction of enemy tank if it reached the screen edge
pub fn collision_with_field_edges(
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
            PI + FRAC_PI_2
        } else if translation.y > y_max {
            translation.y = y_max;
            FRAC_PI_2
        } else { 0. };// will bever happen
        transform.translation = translation;

        let target_quat = azimuth_to_quat_negative_z(rotation_angle_relative + rotation_angle_shift);
        enemy.start_rotate(target_quat);
    }
}

/// Rotates tanks if they collide with other tanks
pub fn collision_with_tanks(
    mut query: Query<(&Transform, &mut Enemy, Entity), With<Enemy>>,
) {
    let mut collisions: HashMap<Entity, Quat> = HashMap::new();
    for [(transform1, enemy1, entity1), (transform2, enemy2, entity2)] in query.iter_combinations() {
        if transform1.translation.distance(transform2.translation) > constants::TANK_DIMENSION {
            continue
        }


        // TODO: randomize rotation angles for both tanks
        // let vec_between = transform1.translation - transform2.translation;
        // transform1.rotate(rotation)

        if !(enemy1.is_rotating() || collisions.contains_key(&entity1)) {
            collisions.insert(entity1, azimuth_to_quat_negative_z(90.));
        }
        if !(enemy2.is_rotating() || collisions.contains_key(&entity2)) {
            collisions.insert(entity2, azimuth_to_quat_negative_z(90.));
        }
    }

    for (transform, mut enemy, entity) in query.iter_mut() {
        if !collisions.contains_key(&entity) {
            continue
        }

        let quat = collisions.get(&entity).unwrap();
        enemy.start_rotate(quat.clone());
    }
}