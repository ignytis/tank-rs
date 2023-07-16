use bevy::prelude::*;

use crate::components::player::Player;
use crate::components::spawn_player::SpawnPlayer;

/// Spawns player's tank on player spawn
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
            transform: Transform::from_xyz(pt.x, pt.y, 1.).with_scale(Vec3::splat(0.25)),
            texture: asset_server.load("sprites/tank.png"),
            ..default()
        },
        Player{},
    ));
    player_spawn.timer.reset();
}