use bevy::prelude::*;

#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum GameState {
    GameOver,
    PlayerWon,
    #[default]
    Running,
}