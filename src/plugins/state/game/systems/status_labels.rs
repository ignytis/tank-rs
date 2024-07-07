use bevy::prelude::*;

use crate::{
    constants::COLOR_RED,
    plugins::state::game::components::hud::StatusLabel};

pub fn spawn_player_won_label(
    asset_server: Res<AssetServer>,
    commands: Commands,
) {
    spawn_status_label(asset_server, commands, format!("You won!"));
}

pub fn spawn_game_over_label(
    asset_server: Res<AssetServer>,
    commands: Commands,
) {
    spawn_status_label(asset_server, commands, format!("Game Over"));
}

pub fn spawn_paused_label(
    asset_server: Res<AssetServer>,
    commands: Commands,
) {
    spawn_status_label(asset_server, commands, format!("Paused"));
}

/// Despawns the status (e.g. "paused" label)
pub fn despawn_status_label(
    mut commands: Commands,
    query: Query<Entity, With<StatusLabel>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn spawn_status_label(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    label: String
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            StatusLabel::default()))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            label,
                            TextStyle {
                                font: asset_server.load("fonts/hobby-of-night.ttf"),
                                font_size: 200.0,
                                color: COLOR_RED,  
                            },
                        )
                    );
                });
        });
}