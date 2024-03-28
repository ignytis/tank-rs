use bevy::prelude::*;

use crate::plugins::state::menu::constants;
use crate::plugins::state::menu::components::{MenuBlock, MenuItem, MenuItemType};

fn get_button(parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>, item_type: MenuItemType, label: &str) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Style::DEFAULT
                },
                border_color: BorderColor(Color::BLACK),
                background_color: constants::COLOR_BUTTON.into(),
                ..default()
            },
            MenuItem::new(item_type)
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: asset_server.load("fonts/hobby-of-night.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                display: Display::Flex,
                ..default()
            },
            ..default()
        }, MenuBlock::default()))
        .with_children(|parent| {
            get_button(parent, &asset_server, MenuItemType::NewGame, "New game");
            get_button(parent, &asset_server, MenuItemType::LevelEditor, "Level editor");
            get_button(parent, &asset_server, MenuItemType::Exit, "Exit");
        });
}