mod components;
mod resources;
mod states;
mod systems;

use bevy::prelude::*;

use crate::plugins::state::game::systems::animation as systems_animation;
use crate::plugins::state::game::systems::enemy_movement as systems_enemy_movement;
use crate::plugins::state::game::systems::game_status as systems_game_status;
use crate::plugins::state::game::systems::player_movement as systems_player_movement;
use crate::plugins::state::game::systems::setup as systems_setup;
use crate::plugins::state::game::systems::shells as systems_shells;
use crate::plugins::state::game::systems::shutdown as systems_shutdown;
use crate::plugins::state::game::systems::simulation_state as systems_simulation_state;
use crate::plugins::state::game::systems::status_labels as systems_status_labels;
use crate::plugins::state::game::systems::spawn as systems_spawn;

use crate::states::SceneState;
use crate::plugins::state::game::resources::lives::Lives;
use crate::plugins::state::game::states::{GameState, SimulationState};


/// Game window
pub struct GamePlugin;


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::InGame), (
            systems_setup::init_state,
            systems_setup::add_ground,
            systems_setup::add_player_spawn,
            systems_setup::add_enemy_spawn,
            systems_setup::add_hud,
            // systems_setup::add_walls, // TODO: implement collisions and uncomment
        ).chain())
        .add_systems(Update, (
            systems_simulation_state::quit_to_main_menu,
        ).run_if(in_state(SceneState::InGame)))
        .add_systems(Update, (
            systems_simulation_state::pause_and_resume_game,
        ).run_if(in_state(SceneState::InGame)
            .and_then(in_state(GameState::Running))))
        .add_systems(Update, (
            systems_animation::animate_sprite,
            systems_spawn::spawn_enemy,
            systems_enemy_movement::move_enemies,
            systems_enemy_movement::collision_with_field_edges,
            systems_enemy_movement::collision_with_tanks,
            systems_shells::enemy_shoot,
            systems_shells::shell_move,
            systems_shells::shell_offscreen_despawn,
            systems_shells::tank_hit_enemy,
        ).run_if(in_state(SceneState::InGame)
            .and_then(in_state(SimulationState::Running))))
        .add_systems(Update, (
            systems_player_movement::confine_player_movement,
            systems_player_movement::move_player,
            systems_shells::player_shoot,
            systems_shells::tank_hit_player,
            systems_spawn::spawn_player,
            systems_game_status::check_tanks,
        ).run_if(in_state(SceneState::InGame)
            .and_then(in_state(GameState::Running))
            .and_then(in_state(SimulationState::Running))))
        .add_systems(OnEnter(GameState::GameOver), (
            systems_status_labels::spawn_game_over_label,
        ).run_if(in_state(SceneState::InGame)))
        .add_systems(OnEnter(GameState::PlayerWon), (
            systems_status_labels::spawn_player_won_label,
        ).run_if(in_state(SceneState::InGame)))
        .add_systems(OnEnter(SimulationState::Paused), (
            systems_status_labels::spawn_paused_label,
        ).run_if(in_state(SceneState::InGame)))
        .add_systems(OnEnter(SimulationState::Running), (
            systems_status_labels::despawn_status_label,
        ).run_if(in_state(SceneState::InGame)))
        .add_systems(OnExit(SceneState::InGame), (
            systems_shutdown::remove_components,
        ))
        .insert_resource(Lives::default())
        .add_state::<GameState>()
        .add_state::<SimulationState>();
    }
}