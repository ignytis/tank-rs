use bevy::prelude::*;

use crate::plugins::state::menu::constants;
use crate::states::SceneState;

pub fn button_events(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor
        ),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = constants::COLOR_BUTTON_PRESSED.into();
                border_color.0 = Color::RED;
                commands.insert_resource(NextState(Some(SceneState::InGame)));
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = constants::COLOR_BUTTON_HOVERED.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = constants::COLOR_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}