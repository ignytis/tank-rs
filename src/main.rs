mod constants;
mod geometry;
mod plugins;
mod states;
mod systems_global;

use bevy::prelude::*;

use crate::plugins::state::game::GamePlugin;
use crate::plugins::state::menu::MenuPlugin;

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

        bevy_framepace::FramepacePlugin,

        GamePlugin,
        MenuPlugin,
    ))
    .init_state::<states::SceneState>()
    .add_systems(Startup, (
        systems_global::setup_window,
        systems_global::setup_framerate,
    ))
    .run();
}