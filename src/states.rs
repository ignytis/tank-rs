use bevy::prelude::*;

/// Scene
#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
pub enum SceneState {
    InGame,
    LevelEditor,
    #[default]
    MainMenu,
}