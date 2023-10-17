use bevy::prelude::*;

use crate::plugins::state::game::states::SimulationState;

pub fn pause_and_resume_game(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    state: Res<State<SimulationState>>,
) {
    if !input.just_pressed(KeyCode::Escape) {
        return
    }

    let new_state = match state.get() {
        SimulationState::Paused => SimulationState::Running,
        SimulationState::Running => SimulationState::Paused,
    };

    commands.insert_resource(NextState(Some(new_state)));
}