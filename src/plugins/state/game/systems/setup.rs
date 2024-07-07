use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::plugins::state::game::components::hud::{EnemyLives, PlayerLives};

use crate::plugins::state::game::components::spawn_player::SpawnPlayer;
use crate::plugins::state::game::components::spawn_enemy::SpawnEnemy;
use crate::plugins::state::game::components::map::Ground;
use crate::constants;

use crate::plugins::state::game::resources::lives::Lives;
use crate::plugins::state::game::states::{GameState, SimulationState};

const FLOOR_TEXTURE_HEIGHT: f32 = 300.;
const FLOOR_TEXTURE_WIDTH: f32 = 300.;

pub fn init_state(
    mut commands: Commands,
) {
    commands.insert_resource(NextState::Pending(SimulationState::default()));
    commands.insert_resource(NextState::Pending(GameState::default()));
    commands.insert_resource(Lives::default());
}

pub fn add_player_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz( constants::TANK_DIMENSION as f32 / 2., constants::WINDOW_HEIGHT / -2. + constants::TANK_DIMENSION as f32, constants::Z_INDEX_SPAWN),
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
                transform: Transform::from_xyz( constants::TANK_DIMENSION as f32 / 2. + constants::WINDOW_WIDTH as f32 / 2.5 * i as f32,
                    constants::WINDOW_HEIGHT / 2. - constants::TANK_DIMENSION as f32, constants::Z_INDEX_SPAWN),
                texture: asset_server.load("sprites/spawn_enemy.png"),
                ..default()
            },
            SpawnEnemy::default(),
        ));
    }

}

/// Adds a ground texture
pub fn add_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let asset = asset_server.load("sprites/ground.png");
    // let repeat_assets_x = (constants::WINDOW_WIDTH / FLOOR_TEXTURE_WIDTH).ceil() as u32;
    // let repeat_assets_y = (constants::WINDOW_HEIGHT / FLOOR_TEXTURE_HEIGHT).ceil() as u32;

    // for x in 0..repeat_assets_x + 1 {
    //     for y in 0..repeat_assets_y + 1 {
    //         commands.spawn((
    //             SpriteBundle {
    //                 transform: Transform::from_xyz( constants::WINDOW_WIDTH / -2. + FLOOR_TEXTURE_WIDTH * x as f32,
    //                     constants::WINDOW_HEIGHT / -2. + FLOOR_TEXTURE_HEIGHT * y as f32, constants::Z_INDEX_GROUND),
    //                 texture: asset.clone(),
    //                 ..default()
    //             },
    //             Ground::default(),
    //         ));
    //     }
    // }
}

pub fn add_hud(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    lives: Res<Lives>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands
        .spawn(NodeBundle{ // Main container
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // align_items: AlignItems::Center,
                // justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle{ // Game area
                    // background_color: BackgroundColor::from(Color::WHITE),
                    style: Style {
                        // flex_grow: 4.,
                        flex_basis: Val::Percent(75.),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    // let asset = asset_server.load("sprites/ground.png");
                    // let repeat_assets_x = (constants::WINDOW_WIDTH / FLOOR_TEXTURE_WIDTH).ceil() as u32;
                    // let repeat_assets_y = (constants::WINDOW_HEIGHT / FLOOR_TEXTURE_HEIGHT).ceil() as u32;

                    
                    // for x in 0..repeat_assets_x + 1 {
                    //     for y in 0..repeat_assets_y + 1 {
                    //         parent.spawn((
                    //             SpriteBundle {
                    //                 transform: Transform::from_xyz( constants::WINDOW_WIDTH / -2. + FLOOR_TEXTURE_WIDTH * x as f32,
                    //                     constants::WINDOW_HEIGHT / -2. + FLOOR_TEXTURE_HEIGHT * y as f32, constants::Z_INDEX_GROUND),
                    //                 texture: asset.clone(),
                    //                 ..default()
                    //             },
                    //             Ground::default(),
                    //         ));
                    //     }
                    // }
                });

            parent
                .spawn(NodeBundle{ // HUD
                    background_color: BackgroundColor::from(Color::BLACK),
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        flex_basis: Val::Percent(25.),
                        // flex_grow: 1.,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        // Create a TextBundle that has a Text with a single section.
                        TextBundle::from_section(
                            // Accepts a `String` or any type that converts into a `String`, such as `&str`
                            format!("Enemy lives: {}", lives.enemy_lives),
                            TextStyle {
                                font: asset_server.load("fonts/hobby-of-night.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE,
                            },
                        ) // Set the alignment of the Text
                        .with_text_justify(JustifyText::Center)
                        // Set the style of the TextBundle itself.
                        .with_style(Style {
                            // flex_grow: 1.,
                            // position_type: PositionType::Absolute,
                            // bottom: Val::Px(105.0),
                            // right: Val::Px(5.0),
                            ..default()
                        }),
                        EnemyLives::default(),
                    ));
                
                    parent.spawn((
                        // Create a TextBundle that has a Text with a single section.
                        TextBundle::from_section(
                            // Accepts a `String` or any type that converts into a `String`, such as `&str`
                            format!("Player's lives: {}", lives.player_lives),
                            TextStyle {
                                font: asset_server.load("fonts/hobby-of-night.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE,
                            },
                        ) // Set the alignment of the Text
                        .with_text_justify(JustifyText::Center)
                        // Set the style of the TextBundle itself.
                        .with_style(Style {
                            // flex_grow: 1.,
                            // position_type: PositionType::Absolute,
                            // bottom: Val::Px(5.0),
                            // right: Val::Px(5.0),
                            ..default()
                        }),
                        PlayerLives::default(),
                    ));
                });
        });

    // commands.spawn((
    //     // Create a TextBundle that has a Text with a single section.
    //     TextBundle::from_section(
    //         // Accepts a `String` or any type that converts into a `String`, such as `&str`
    //         format!("Enemy lives: {}", lives.enemy_lives),
    //         TextStyle {
    //             font: asset_server.load("fonts/hobby-of-night.ttf"),
    //             font_size: 100.0,
    //             color: Color::WHITE,
    //         },
    //     ) // Set the alignment of the Text
    //     .with_text_justify(JustifyText::Center)
    //     // Set the style of the TextBundle itself.
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         bottom: Val::Px(105.0),
    //         right: Val::Px(5.0),
    //         ..default()
    //     }),
    //     EnemyLives::default(),
    // ));

    // commands.spawn((
    //     // Create a TextBundle that has a Text with a single section.
    //     TextBundle::from_section(
    //         // Accepts a `String` or any type that converts into a `String`, such as `&str`
    //         format!("Player's lives: {}", lives.player_lives),
    //         TextStyle {
    //             font: asset_server.load("fonts/hobby-of-night.ttf"),
    //             font_size: 100.0,
    //             color: Color::WHITE,
    //         },
    //     ) // Set the alignment of the Text
    //     .with_text_justify(JustifyText::Center)
    //     // Set the style of the TextBundle itself.
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         bottom: Val::Px(5.0),
    //         right: Val::Px(5.0),
    //         ..default()
    //     }),
    //     PlayerLives::default(),
    // ));
}