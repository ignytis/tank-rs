use std::time::Duration;

use bevy::prelude::*;

/// Player spawn
#[derive(Component)]
pub struct SpawnPlayer {
    pub timer: Timer,
}

impl Default for SpawnPlayer {
    /// Initialized a spawn with timer already expired to spawn a tank instantly on startup
    fn default() -> Self {
        let duration = Duration::from_secs(2);
        let mut timer = Timer::new(duration, TimerMode::Once);
        timer.set_elapsed(duration);

        SpawnPlayer { timer }
    }
}