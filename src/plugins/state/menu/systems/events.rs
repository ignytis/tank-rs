use bevy::prelude::*;

use crate::plugins::state::menu::components::{MenuItem, MenuItemType};
use crate::plugins::state::menu::constants;
use crate::states::SceneState;

pub fn button_events(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &MenuItem
        ),
        (Changed<Interaction>, With<MenuItem>),
    >,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>
) {
    for (interaction, mut color, mut border_color,menu_item) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = constants::COLOR_BUTTON_PRESSED.into();
                border_color.0 = Color::RED;
                match menu_item.item_type {
                    MenuItemType::NewGame => commands.insert_resource(NextState(Some(SceneState::InGame))),
                    MenuItemType::LevelEditor => commands.insert_resource(NextState(Some(SceneState::LevelEditor))),
                    MenuItemType::Exit => app_exit_events.send(bevy::app::AppExit),
                };
            }
            Interaction::Hovered => {
                *color = constants::COLOR_BUTTON_HOVERED.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = constants::COLOR_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
