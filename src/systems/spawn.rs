use bevy::prelude::*;

use crate::components::player::Player;
use crate::components::enemy::Enemy;
use crate::components::spawn_enemy::SpawnEnemy;
use crate::components::spawn_player::SpawnPlayer;

use crate::constants::Z_INDEX_TANK;

/// Spawns player tank on spawn
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_spawn_query: Query<(&mut SpawnPlayer, &Transform), With<SpawnPlayer>>,
    time: Res<Time>
) {
    let (mut player_spawn, player_spawn_transform) = player_spawn_query.single_mut();
    player_spawn.timer.tick(time.delta());

    if !player_spawn.timer.finished() {
        return;
    }

    let pt = player_spawn_transform.translation;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(pt.x, pt.y, Z_INDEX_TANK),
            texture: asset_server.load("sprites/tank_player.png"),
            ..default()
        },
        Player{},
    ));
    player_spawn.timer.reset();
}

/// Spawns enemy tanks on spawn
pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_query: Query<(&mut SpawnEnemy, &Transform), With<SpawnEnemy>>,
    time: Res<Time>
) {
    for (mut enemy_spawn, enemy_spawn_transform) in enemy_spawn_query.iter_mut() {
        enemy_spawn.timer.tick(time.delta());

        if !enemy_spawn.timer.finished() {
            continue;
        }
    
        let pt = enemy_spawn_transform.translation;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pt.x, pt.y, Z_INDEX_TANK),
                texture: asset_server.load("sprites/tank_enemy.png"),
                ..default()
            },
            Enemy{},
        ));
        enemy_spawn.timer.reset();
    }
}