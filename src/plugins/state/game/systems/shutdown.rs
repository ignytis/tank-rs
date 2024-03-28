use bevy::prelude::*;

use crate::plugins::state::game::components::animation::{AnimationData, AnimationTimer};
use crate::plugins::state::game::components::enemy::Enemy;
use crate::plugins::state::game::components::hud::{EnemyLives, PlayerLives, StatusLabel};
use crate::plugins::state::game::components::map::Ground;
use crate::plugins::state::game::components::player::Player;
use crate::plugins::state::game::components::shell::{PlayerShell, EnemyShell};
use crate::plugins::state::game::components::spawn_enemy::SpawnEnemy;
use crate::plugins::state::game::components::spawn_player::SpawnPlayer;

pub fn remove_components(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<AnimationData>, With<AnimationTimer>, With<Enemy>,
        With<EnemyLives>, With<Ground>, With<PlayerLives>, With<StatusLabel>, With<SpawnEnemy>,
        With<SpawnPlayer>, With<Player>, With<PlayerShell>, With<EnemyShell>)>>,
) {
    entities.iter().for_each(|e| commands.entity(e).despawn_recursive())
}