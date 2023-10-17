use std::time::Duration;

use bevy::prelude::*;

/// Enemy spawn
#[derive(Component)]
pub struct SpawnEnemy {
    pub timer: Timer,
}

impl Default for SpawnEnemy {
    fn default() -> Self {
        SpawnEnemy { timer: Timer::new(Duration::from_secs(2), TimerMode::Once) }
    }
}