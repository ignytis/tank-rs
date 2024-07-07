use bevy::prelude::*;

/// A shell shot by player
#[derive(Component)]
pub struct PlayerShell {}

impl PlayerShell {
    pub fn new() -> Self {
        PlayerShell {}
    }
}

/// A shell shot by enemy
#[derive(Component)]
pub struct EnemyShell {}

impl EnemyShell {
    pub fn new() -> Self {
        EnemyShell {}
    }
}