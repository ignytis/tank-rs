use bevy::prelude::*;

use crate::{plugins::state::game::states::SimulationState, states::SceneState};

pub fn pause_and_resume_game(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimulationState>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return
    }

    let new_state = match state.get() {
        SimulationState::Paused => SimulationState::Running,
        SimulationState::Running => SimulationState::Paused,
    };

    commands.insert_resource(NextState::Pending(new_state));
}

pub fn quit_to_main_menu(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::Escape) {
        return
    }

    commands.insert_resource(NextState::Pending(SceneState::MainMenu));
}