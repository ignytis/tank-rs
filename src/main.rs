mod components;
mod systems;

use bevy::prelude::*;

use crate::systems::confine_player_movement as systems_confine_player_movement;
use crate::systems::setup as systems_setup;
use crate::systems::keyboard as systems_keyboard;

fn main(){
  App::new()
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tanks-rs".into(),
                resolution: (1024., 768.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }),
    ))
    .add_systems(Startup, (
        systems_setup::setup_window,
        systems_setup::spawn_player,
    ))
    .add_systems(Update, (
        systems_keyboard::keyboard_events,
        systems_confine_player_movement::keep_player_in_window,
    ))
    .run();
}