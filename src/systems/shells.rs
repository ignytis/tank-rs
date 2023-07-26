use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::enemy::Enemy;
use crate::components::player::Player;
use crate::components::shell::Shell;

use crate::constants::Z_INDEX_SHELL;


const SHELL_SPEED: f32 = 10.;

/// Enemy tank shooting
pub fn enemy_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_query: Query<(&mut Enemy, &Transform), With<Enemy>>,
    time: Res<Time>
) {    
    for (mut enemy, enemy_transform) in enemy_query.iter_mut() {    
        let transl = enemy_transform.translation;
        
        enemy.shoot_timer.tick(time.delta());
        if !enemy.shoot_timer.finished() {
            continue;
        }

        let shell = Shell::new_enemy(enemy.azimuth);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(transl.x, transl.y, Z_INDEX_SHELL)
                    .with_rotation(Quat::from_axis_angle(Vec3::new(0., 0., -1.), enemy.azimuth)), // randomize direction
                texture: asset_server.load("sprites/shell.png"),
                ..default()
            },
            shell,
        ));
    }
}

/// Handles the shoot button click
pub fn player_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut player, player_transform) = match player_query.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    if !player.shoot_timer.paused() {
        player.shoot_timer.tick(time.delta());
    }

    if !(input.pressed(KeyCode::ControlLeft) && player.shoot_timer.finished()) {
        return;
    }

    let azimuth = player.azimuth;
    let shell = Shell::new_player(azimuth);
    let transl = player_transform.translation;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(transl.x, transl.y, Z_INDEX_SHELL)
                .with_rotation(Quat::from_axis_angle(Vec3::new(0., 0., -1.), azimuth)), // randomize direction
            texture: asset_server.load("sprites/shell.png"),
            ..default()
        },
        shell,
    ));

    player.shoot_timer.reset();
}

/// Makes shells move forward
pub fn shell_move(
    mut query: Query<&mut Transform, With<Shell>>
) {
    for mut transform in query.iter_mut() {    
        let v = transform.rotation * Vec3::Y * SHELL_SPEED;
        transform.translation += v;
    }
}

/// Despawns shells which moved out of screen
pub fn shell_offscreen_despawn(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Shell>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = window.width() / -2.0;
    let x_max = window.width() / 2.0;
    let y_min = window.height() / -2.0;
    let y_max = window.height() / 2.0;

    for (entity, transform) in query.iter_mut() {    
        let t = transform.translation;
        if t.x < x_min || t.x > x_max || t.y < y_min || t.y > y_max {
            commands.entity(entity).despawn();
        }
    }
}