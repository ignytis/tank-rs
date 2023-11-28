use bevy::prelude::*;

pub fn setup_window(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}