use bevy::prelude::*;

use crate::components::hud::StatusLabel;

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

/// Player's tank hits by enemy shells
pub fn despawn_status_label(
    mut commands: Commands,
    query: Query<Entity, With<StatusLabel>>,
) {
    match query.get_single() {
        Ok(entity) => commands.entity(entity).despawn(),
        _ => {},
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
            StatusLabel{}))
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
                    parent.spawn((
                        TextBundle::from_section(
                            label,
                            TextStyle {
                                font: asset_server.load("fonts/hobby-of-night.ttf"),
                                font_size: 200.0,
                                color: Color::RED,  
                            },
                        ),
                        Label,
                    ));
                });
        });
}