use bevy::prelude::*;

use crate::components::spawn_player::SpawnPlayer;
use crate::constants;

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
            transform: Transform::from_xyz( constants::TANK_DIMENSION/2., constants::WINDOW_HEIGHT / -2. + constants::TANK_DIMENSION, constants::Z_INDEX_SPAWN),
            texture: asset_server.load("sprites/spawn_player.png"),
            ..default()
        },
        SpawnPlayer::default(),
    ));
}