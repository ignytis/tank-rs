use bevy::prelude::*;

use crate::constants::MAX_ENEMIES;
use crate::components::enemy::Enemy;

pub fn not_all_enemies_spawned(
    query: Query<&Enemy>,
) -> bool {
    query.iter().len() <= MAX_ENEMIES
}