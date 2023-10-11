use bevy::prelude::*;

pub fn spawn_game_over_label(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
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
                            format!("Game Over"),
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