use bevy::prelude::*;

/// A shell shot by player or enemy
#[derive(Component)]
pub struct Shell {
    pub azimuth: f32,
    pub shot_by: bool, // false - enemy, true - player
}

impl Shell {
    /// Spawns enemy's shell
    pub fn new_enemy(azimuth: f32) -> Self {
        Shell {
            azimuth,
            shot_by: false,
            ..default()
        }
    }

    /// Spawns player's shell
    pub fn new_player(azimuth: f32) -> Self {
        Shell {
            azimuth,
            shot_by: true,
            ..default()
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            azimuth: 0.,
            shot_by: true,
        }
    }
}