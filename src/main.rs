mod components;
mod systems;

use bevy::prelude::*;

use crate::systems::setup as systems_setup;
use crate::systems::keyboard as systems_keyboard;

fn main(){
  App::new()
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tanks-rs".into(),
                resolution: (1024., 768.).into(),
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
    ))
    .run();
}