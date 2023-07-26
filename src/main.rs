mod components;
mod conditions;
mod constants;
mod geometry;
mod systems;

use bevy::prelude::*;

use crate::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};

use crate::systems::confine_player_movement as systems_confine_player_movement;
use crate::systems::enemy_movement as systems_enemy_movement;
use crate::systems::setup as systems_setup;
use crate::systems::shells as systems_shells;
use crate::systems::spawn as systems_spawn;
use crate::systems::keyboard as systems_keyboard;

use crate::conditions::enemy as enemy_conditions;
use crate::conditions::player as player_conditions;

fn main(){
  App::new()
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tanks-rs".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }),
    ))
    .add_systems(Startup, (
        systems_setup::setup_window,
        systems_setup::add_player_spawn,
        systems_setup::add_enemy_spawn,
    ))
    .add_systems(Update, (
        systems_keyboard::keyboard_events,
        systems_confine_player_movement::keep_player_in_window,
    ).run_if(player_conditions::is_player_spawned))
    .add_systems(Update, (
        systems_spawn::spawn_player,
    ).run_if(player_conditions::is_not_player_spawned))
    .add_systems(Update, (
        systems_spawn::spawn_enemy,
    ).run_if(enemy_conditions::not_all_enemies_spawned))
    .add_systems(Update, (
        systems_enemy_movement::move_enemies,
        systems_enemy_movement::change_enemy_direction,
        systems_shells::enemy_shoot,
        systems_shells::player_shoot,
        systems_shells::shell_move,
        systems_shells::shell_offscreen_despawn,
    ))
    .run();
}