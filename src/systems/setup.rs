use bevy::prelude::*;

use crate::components::spawn_player::SpawnPlayer;

pub fn setup_window(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

pub fn add_player_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz( 0.0, -450.0, 0.0).with_scale(Vec3::splat(0.25)),
            texture: asset_server.load("sprites/spawn_player.png"),
            ..default()
        },
        SpawnPlayer::default(),
    ));
}