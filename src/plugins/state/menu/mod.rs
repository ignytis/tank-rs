pub mod components;
pub mod constants;
pub mod systems;

use bevy::prelude::*;

use crate::plugins::state::menu::systems::events as systems_events;
use crate::plugins::state::menu::systems::setup as systems_setup;
use crate::plugins::state::menu::systems::shutdown as systems_shutdown;
use crate::states::SceneState;

/// Game window
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SceneState::MainMenu), systems_setup::setup)
            .add_systems(Update, systems_events::button_events.run_if(in_state(SceneState::MainMenu)))
            .add_systems(OnExit(SceneState::MainMenu), systems_shutdown::shutdown)
        ;
        
    }
}