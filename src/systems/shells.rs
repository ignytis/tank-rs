use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::enemy::Enemy;
use crate::components::player::Player;
use crate::components::shell::{PlayerShell, EnemyShell};

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

        let shell = EnemyShell::new(enemy.azimuth);
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
    let shell = PlayerShell::new(azimuth);
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
    mut p_q: Query<&mut Transform, (With<PlayerShell>, Without<EnemyShell>)>,
    mut e_q: Query<&mut Transform, (With<EnemyShell>, Without<PlayerShell>)>,
) {
    for mut transform in p_q.iter_mut() {    
        let v = transform.rotation * Vec3::Y * SHELL_SPEED;
        transform.translation += v;
    }

    for mut transform in e_q.iter_mut() {    
        let v = transform.rotation * Vec3::Y * SHELL_SPEED;
        transform.translation += v;
    }
}

/// Despawns shells which moved out of screen
pub fn shell_offscreen_despawn(
    mut commands: Commands,
    mut p_q: Query<(Entity, &Transform), With<PlayerShell>>,
    mut e_q: Query<(Entity, &Transform), With<EnemyShell>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = window.width() / -2.0;
    let x_max = window.width() / 2.0;
    let y_min = window.height() / -2.0;
    let y_max = window.height() / 2.0;

    // Despawn player shells
    for (entity, transform) in p_q.iter_mut() {    
        let t = transform.translation;
        if t.x < x_min || t.x > x_max || t.y < y_min || t.y > y_max {
            commands.entity(entity).despawn();
        }
    }

    // Despawn enemy shells
    for (entity, transform) in e_q.iter_mut() {    
        let t = transform.translation;
        if t.x < x_min || t.x > x_max || t.y < y_min || t.y > y_max {
            commands.entity(entity).despawn();
        }
    }
}

pub fn tank_hit(
    mut commands: Commands,
    mut player_shell_query: Query<(Entity, &Transform), With<PlayerShell>>,
    mut enemy_shell_query: Query<(Entity, &Transform), With<EnemyShell>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (player_shell_entity, player_shell_transform) in player_shell_query.iter_mut() {    
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
            if player_shell_transform.translation.distance(enemy_transform.translation) > 40. {
                continue
            }
            commands.entity(player_shell_entity).despawn();
            commands.entity(enemy_entity).despawn();
        }
    }

    for (enemy_shell_entity, enemy_shell_transform) in enemy_shell_query.iter_mut() {    
        for (player_entity, player_transform) in player_query.iter_mut() {
            if enemy_shell_transform.translation.distance(player_transform.translation) > 40. {
                continue
            }
            commands.entity(enemy_shell_entity).despawn();
            commands.entity(player_entity).despawn();
        }
    }
}