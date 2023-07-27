use bevy::prelude::*;

/// A shell shot by player
#[derive(Component)]
pub struct PlayerShell {
    pub azimuth: f32,
}

impl PlayerShell {
    pub fn new(azimuth: f32) -> Self {
        PlayerShell {
            azimuth,
        }
    }
}

/// A shell shot by enemy
#[derive(Component)]
pub struct EnemyShell {
    pub azimuth: f32,
}

impl EnemyShell {
    pub fn new(azimuth: f32) -> Self {
        EnemyShell {
            azimuth,
        }
    }
}