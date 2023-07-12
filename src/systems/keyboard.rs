use bevy::prelude::*;

use crate::components::player::Player;

pub fn keyboard_events(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, With<Player>)>,
) {
    let (mut transform, _) = player_query.single_mut();
    if input.pressed(KeyCode::Up) {
        let v = transform.rotation * Vec3::Y;
        transform.translation += v;
    } else if input.pressed(KeyCode::Down) {
        let v = transform.rotation * Vec3::Y * -1.0;
        transform.translation += v;
    } else if input.pressed(KeyCode::Left) {
        transform.rotate(Quat::from_rotation_z((1.0_f32).to_radians()));
    } else if input.pressed(KeyCode::Right) {
        transform.rotate(Quat::from_rotation_z((-1.0_f32).to_radians()));
    }    
}