use bevy::prelude::*;

use crate::components::player::Player;

const MOVEMENT_FACTOR_BACKWARD: f32 = 1.3;
const MOVEMENT_FACTOR_FORWARD: f32 = 3.0;
const ROTATION_FACTOR: f32 = 2.0;

pub fn keyboard_events(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, With<Player>)>,
) {
    // No player is spawned
    if player_query.is_empty() {
        return;
    }

    let (mut transform, _) = player_query.single_mut();
    if input.pressed(KeyCode::Up) {
        let v = transform.rotation * Vec3::Y * MOVEMENT_FACTOR_FORWARD;
        transform.translation += v;
    } else if input.pressed(KeyCode::Down) {
        let v = transform.rotation * Vec3::Y * MOVEMENT_FACTOR_BACKWARD * -1.0;
        transform.translation += v;
    } 

    if input.pressed(KeyCode::Left) {
        transform.rotate(Quat::from_rotation_z((1.0_f32 * ROTATION_FACTOR).to_radians()));
    } else if input.pressed(KeyCode::Right) {
        transform.rotate(Quat::from_rotation_z((-1.0_f32 * ROTATION_FACTOR).to_radians()));
    }    
}