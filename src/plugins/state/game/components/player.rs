use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub azimuth: f32,
    pub shoot_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            azimuth: 0.,
            shoot_timer: Timer::new(Duration::from_secs(1), TimerMode::Once)
        }
    }
}