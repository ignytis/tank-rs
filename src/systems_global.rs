use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};

use crate::constants;

pub fn setup_window(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_framerate(
    mut settings: ResMut<FramepaceSettings>
) {
    settings.limiter = Limiter::from_framerate(constants::FRAMERATE_MAX);
}