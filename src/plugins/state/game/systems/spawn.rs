use std::f64::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use crate::plugins::state::game::components::hud::{PlayerLives, EnemyLives};
use crate::plugins::state::game::components::player::Player;
use crate::plugins::state::game::components::enemy::Enemy;
use crate::plugins::state::game::components::spawn_enemy::SpawnEnemy;
use crate::plugins::state::game::components::spawn_player::SpawnPlayer;

use crate::constants::{MAX_ENEMIES, Z_INDEX_TANK};
use crate::plugins::state::game::resources::lives::Lives;

/// Spawns player tank on spawn
pub fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    time: Res<Time>,
    mut hud_hits_query: Query<&mut Text, With<PlayerLives>>,
    mut lives: ResMut<Lives>,
    mut player_spawn_query: Query<(&mut SpawnPlayer, &Transform), With<SpawnPlayer>>,
    player_query: Query<&Player>,
) {
    if !player_query.is_empty() { // if player is already spawned
        return;
    }

    let (mut player_spawn, player_spawn_transform) = player_spawn_query.single_mut();
    player_spawn.timer.tick(time.delta());

    if !player_spawn.timer.finished() {
        return;
    }

    let transl = player_spawn_transform.translation;
    let player = Player::default();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(transl.x, transl.y, Z_INDEX_TANK)
                .with_rotation(Quat::from_axis_angle(Vec3::NEG_Z, player.azimuth)),
            texture: asset_server.load("sprites/tank_player.png"),
            ..default()
        },
        player,
    ));
    player_spawn.timer.reset();

    lives.player_lives -= 1;
    let mut text = hud_hits_query.single_mut();
    text.sections[0].value = format!("Player's lives: {}", lives.player_lives);
}

/// Spawns enemy tanks on spawn
pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_query: Query<(&mut SpawnEnemy, &Transform), With<SpawnEnemy>>,
    enemy_query: Query<&Enemy>,
    mut hud_hits_query: Query<&mut Text, With<EnemyLives>>,
    mut lives: ResMut<Lives>,
    time: Res<Time>,
) {
    let mut num_enemies = enemy_query.iter().len();
    if num_enemies >= MAX_ENEMIES { // all enemies are already spawned
        return
    }

    let mut rng = rand::thread_rng();
    
    for (mut enemy_spawn, enemy_spawn_transform) in enemy_spawn_query.iter_mut() {
        if num_enemies >= MAX_ENEMIES || 0 == lives.enemy_lives {
            return
        }

        enemy_spawn.timer.tick(time.delta());
        if !enemy_spawn.timer.finished() {
            continue;
        }
    
        let transl = enemy_spawn_transform.translation;
        let mut enemy = Enemy::default();
        enemy.azimuth = rng.gen_range((0. as f32)..(TAU as f32));
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(transl.x, transl.y, Z_INDEX_TANK)
                    .with_rotation(Quat::from_axis_angle(Vec3::NEG_Z, enemy.azimuth)), // randomize direction
                texture: asset_server.load("sprites/tank_enemy.png"),
                ..default()
            },
            enemy,
        ));
        num_enemies += 1;
        lives.enemy_lives -= 1;
        let mut text = hud_hits_query.single_mut();
        text.sections[0].value = format!("Enemy lives: {}", lives.enemy_lives);
        enemy_spawn.timer.reset();
    }
}