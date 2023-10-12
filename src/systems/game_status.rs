use bevy::prelude::*;

use crate::{resources::lives::Lives, components::{player::Player, enemy::Enemy}, states::GameState};

/// Ends the game if no tanks left on any side
pub fn check_tanks(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    lives: ResMut<Lives>,
    player_query: Query<Entity, With<Player>>,
) {
    if 0 == lives.player_lives && player_query.is_empty() {
        commands.insert_resource(NextState(Some(GameState::GameOver)));
    }
    if 0 == lives.enemy_lives && enemy_query.is_empty() {
        commands.insert_resource(NextState(Some(GameState::PlayerWon)));
    }
}