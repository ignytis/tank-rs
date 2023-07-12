use bevy::prelude::*;

use crate::components::player::Player;

pub fn setup_window(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz( 0.0, -300.0, 0.0).with_scale(Vec3::splat(0.25)),
            texture: asset_server.load("sprites/tank.png"),
            ..default()
        },
        Player{},
    ));
}