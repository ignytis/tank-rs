use bevy::prelude::*;

use crate::constants;

/// Stores player's and enemy's lives
#[derive(Resource)]
pub struct Lives {
    pub enemy_lives: u8,
    pub player_lives: u8,
}

impl Default for Lives {
    fn default() -> Self {
        Lives {
            enemy_lives: constants::LIVES_ENEMY,
            player_lives: constants::LIVES_PLAYER,
        }
    }
}