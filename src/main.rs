mod constants;
mod geometry;
mod plugins;

use bevy::prelude::*;

use crate::plugins::state::game::GamePlugin;

fn main(){
  App::new()
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tanks-rs".into(),
                resolution: (constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }),
        GamePlugin,
    ))

    .run();
}