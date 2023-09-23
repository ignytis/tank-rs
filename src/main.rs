mod components;
mod constants;
mod geometry;
mod states;
mod systems;

use bevy::prelude::*;

use crate::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};

use crate::systems::enemy_movement as systems_enemy_movement;
use crate::systems::player_movement as systems_player_movement;
use crate::systems::setup as systems_setup;
use crate::systems::shells as systems_shells;
use crate::systems::simulation_state as systems_simulation_state;
use crate::systems::spawn as systems_spawn;

use crate::states::SimulationState;

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
        systems_setup::add_floor,
        // systems_setup::add_walls, // TODO: implement collisions and uncomment
    ))
    .add_systems(Update, (
        systems_spawn::spawn_enemy,
        systems_spawn::spawn_player,
        systems_player_movement::move_player,
        systems_player_movement::confine_player_movement,
        systems_enemy_movement::move_enemies,
        systems_enemy_movement::collision_with_field_edges,
        systems_enemy_movement::collision_with_tanks,
        systems_shells::enemy_shoot,
        systems_shells::player_shoot,
        systems_shells::shell_move,
        systems_shells::shell_offscreen_despawn,
        systems_shells::tank_hit,
    ).run_if(in_state(SimulationState::Running)))
    .add_systems(Update, (
        systems_simulation_state::pause_and_resume_game,
    ))
    .add_state::<SimulationState>()
    .run();
}

