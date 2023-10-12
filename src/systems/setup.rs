use bevy::prelude::*;

use crate::components::hud::{EnemyLives, PlayerLives};

use crate::components::spawn_player::SpawnPlayer;
use crate::components::spawn_enemy::SpawnEnemy;
// use crate::components::wall_unbreakable::WallUnbreakable;  // TODO: implement walls
use crate::constants;

use crate::resources::lives::Lives;

const FLOOR_TEXTURE_HEIGHT: f32 = 300.;
const FLOOR_TEXTURE_WIDTH: f32 = 300.;

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

/// Creates enemy spawns
pub fn add_enemy_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for i in -1..2 {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz( constants::TANK_DIMENSION / 2. + constants::WINDOW_WIDTH / 2.5 * i as f32,
                    constants::WINDOW_HEIGHT / 2. - constants::TANK_DIMENSION, constants::Z_INDEX_SPAWN),
                texture: asset_server.load("sprites/spawn_enemy.png"),
                ..default()
            },
            SpawnEnemy::default(),
        ));
    }

}

/// Adds a floor texture
pub fn add_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let asset = asset_server.load("sprites/floor.png");
    let repeat_assets_x = (constants::WINDOW_WIDTH / FLOOR_TEXTURE_WIDTH).ceil() as u32;
    let repeat_assets_y = (constants::WINDOW_HEIGHT / FLOOR_TEXTURE_HEIGHT).ceil() as u32;

    for x in 0..repeat_assets_x + 1 {
        for y in 0..repeat_assets_y + 1 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz( constants::WINDOW_WIDTH / -2. + FLOOR_TEXTURE_WIDTH * x as f32,
                        constants::WINDOW_HEIGHT / -2. + FLOOR_TEXTURE_HEIGHT * y as f32, constants::Z_INDEX_GROUND),
                    texture: asset.clone(),
                    ..default()
                },
            ));
        }
    }
}

pub fn add_hud(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    lives: Res<Lives>,
) {    commands.spawn((
    // Create a TextBundle that has a Text with a single section.
    TextBundle::from_section(
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        format!("Enemy lives: {}", lives.enemy_lives),
        TextStyle {
            font: asset_server.load("fonts/hobby-of-night.ttf"),
            font_size: 100.0,
            color: Color::WHITE,
        },
    ) // Set the alignment of the Text
    .with_text_alignment(TextAlignment::Center)
    // Set the style of the TextBundle itself.
    .with_style(Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(105.0),
        right: Val::Px(5.0),
        ..default()
    }),
    EnemyLives::default(),
));

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            format!("Player's lives: {}", lives.player_lives),
            TextStyle {
                font: asset_server.load("fonts/hobby-of-night.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        PlayerLives::default(),
    ));
}
// pub fn add_walls(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     let mut transform = Transform::from_xyz( 100., 0., constants::Z_INDEX_WALL);
//     transform.rotate_z(1.15);
//     commands.spawn((
//         SpriteBundle {
//             transform,
//             texture: asset_server.load("sprites/wall_unbreakable_1x2.png"),
//             ..default()
//         },
//         WallUnbreakable::default(),
//     ));
// }