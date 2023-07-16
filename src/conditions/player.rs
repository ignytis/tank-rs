use bevy::prelude::*;

use crate::components::player::Player;

pub fn is_player_spawned(
    player_query: Query<&Player>,
) -> bool {
    !is_not_player_spawned(player_query)
}

pub fn is_not_player_spawned(
    player_query: Query<&Player>,
) -> bool {
    player_query.is_empty()
}